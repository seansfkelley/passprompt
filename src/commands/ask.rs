use rand::seq::SliceRandom;
use rand::thread_rng;
use rpassword::prompt_password_stderr;
use std::iter::FromIterator;
use std::time::SystemTime;

use crate::commands::CommandResult;
use crate::config;
use crate::error::PasspromptError;

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

pub struct Args {
  pub always: bool,
  pub which: Which,
}

pub fn command<'a>(
  config: &mut config::Config,
  args: Args,
) -> Result<CommandResult, Box<dyn std::error::Error>> {
  if config.passwords.len() == 0 {
    return Err(Box::new(PasspromptError::NoPasswordsDefined));
  }

  let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs();

  let tries = config.retries.unwrap_or_default() + 1;

  let success = {
    if let Which::Specific(name) = args.which {
      let entry = config.passwords.get(&name);

      if entry.is_none() {
        eprintln!("no password named {}", name);
        return Ok(CommandResult {
          should_save: false,
          success: false,
        });
      }

      ask_one(tries, &name, entry.unwrap())?
    } else {
      if !args.always {
        match config.last_asked {
          Some(last_asked) => {
            let wait_seconds = config.wait.unwrap_or_default().as_secs();
            if last_asked + wait_seconds >= now {
              return Ok(CommandResult {
                should_save: false,
                success: true,
              });
            }
          }
          None => (),
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
      names.shuffle(&mut thread_rng());
      names.truncate(count);

      let mut success = true;
      for name in names {
        let entry = config.passwords.get(name).unwrap();
        // Due to short-circuiting, this `&&` is in the opposite order I would normally write it.
        success = ask_one(tries, name, entry)? && success;
      }

      success
    }
  };

  config.last_asked = Some(now);

  Ok(CommandResult {
    should_save: true,
    success,
  })
}

fn ask_one(
  mut tries: usize,
  password_name: &String,
  password_entry: &config::PasswordEntry,
) -> Result<bool, Box<dyn std::error::Error>> {
  let mut success = false;

  while tries > 0 {
    let input =
      prompt_password_stderr(format!("[passprompt] password for {}: ", password_name).as_str())?;

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
