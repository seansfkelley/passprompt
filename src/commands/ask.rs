use rand::rng;
use rand::seq::SliceRandom;
use rpassword::prompt_password;
use std::iter::FromIterator;
use std::time::SystemTime;

use crate::commands::CommandResult;
use crate::config;
use crate::error::PasspromptError;
use crate::state;

#[derive(Debug)]
pub enum Which {
  Any(usize),
  All,
  Specific(String),
}

impl Which {
  pub fn from_cli_args(
    count_arg_name: String,
    count: Option<String>,
    name: Option<String>,
  ) -> Result<Which, PasspromptError> {
    if count.is_some() && name.is_some() {
      panic!("cannot specify a count and a name, but this should have been handled earlier");
    } else if let Some(n) = count {
      if n.to_ascii_lowercase() == "all" {
        Ok(Which::All)
      } else {
        n.parse::<usize>().map_or_else(
          |_| {
            Err(PasspromptError::IllegalArgument {
              name: count_arg_name,
              value: n,
            })
          },
          |parsed| Ok(Which::Any(parsed)),
        )
      }
    } else if let Some(n) = name {
      Ok(Which::Specific(n))
    } else {
      Ok(Which::Any(1))
    }
  }
}

#[derive(Debug)]
pub struct Args {
  pub always: bool,
  pub which: Which,
}

pub fn command<'a>(
  config: &mut config::Config,
  state_manager: state::StateManager,
  args: Args,
) -> Result<CommandResult, Box<dyn std::error::Error>> {
  if config.passwords.len() == 0 {
    return Err(Box::new(PasspromptError::NoPasswordsDefined));
  }

  let tries = config.retries.unwrap_or_default() + 1;

  let success = {
    if let Which::Specific(name) = args.which {
      let entry = config.passwords.get(&name);

      if entry.is_none() {
        eprintln!("no password named {}", name);
        return Ok(CommandResult {
          save_config: false,
          success: false,
        });
      }

      ask_one(tries, &name, entry.unwrap())?
    } else {
      if !args.always {
        let result = state_manager.with_state(|state| match state.last_asked {
          Some(last_asked) => {
            let wait_seconds = config.wait.unwrap_or_default().as_secs();
            if last_asked + wait_seconds >= now_in_ms() {
              Ok(Some(CommandResult {
                save_config: false,
                success: true,
              }))
            } else {
              Ok(None)
            }
          }
          None => Ok(None),
        })?;

        if let Some(command_result) = result {
          return Ok(command_result);
        }
      }

      let count = if let Which::Any(n) = args.which {
        n
      } else if let Which::All = args.which {
        config.passwords.len()
      } else {
        panic!("should have handled all Which cases previousl");
      };

      let mut names = Vec::from_iter(config.passwords.keys());
      names.shuffle(&mut rng());
      names.truncate(count);

      let mut success = true;
      for name in names {
        // A bit weird written like this, but the idea is to commit, to disk, the ask time before
        // and after every prompt. We don't know if/when the prompt might be answered, or if the
        // first one is answered and the second is left to idle, but we want to make sure we record
        // the time on any interaction -- when the user did cause them to be asked _or_ when they
        // responded.
        state_manager.with_state(|state| {
          state.last_asked = Some(now_in_ms());
          Ok(())
        })?;

        let response = state_manager.with_state(|state| {
          let entry = config.passwords.get(name).unwrap();
          let result = ask_one(tries, name, entry);
          state.last_asked = Some(now_in_ms());
          result
        })?;

        // Due to short-circuiting, this `&&` is in the opposite order I would normally write it.
        success = response && success;
      }

      success
    }
  };

  Ok(CommandResult {
    save_config: true,
    success,
  })
}

fn now_in_ms() -> u64 {
  SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs()
}

fn ask_one(
  mut tries: usize,
  password_name: &String,
  password_entry: &config::PasswordEntry,
) -> Result<bool, Box<dyn std::error::Error>> {
  let mut success = false;

  while tries > 0 {
    let input = prompt_password(format!("[passprompt] password for {}: ", password_name).as_str())?;

    if input.len() > 0 && password_entry.matches(input) {
      success = true;
      break;
    }

    tries -= 1;
  }

  if success {
    eprintln!("got it!");
  } else {
    eprintln!("better luck next time...");
  }

  Ok(success)
}
