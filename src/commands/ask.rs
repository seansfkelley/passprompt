use rand::seq::SliceRandom;
use rand::thread_rng;
use rpassword::prompt_password_stderr;
use std::iter::FromIterator;
use std::time::SystemTime;

use crate::commands::CommandResult;
use crate::config;
use crate::error::PasspromptError;

pub struct Args {
  pub always: bool,
  pub name: Option<String>,
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

  let (password_name, password_entry) = {
    if let Some(name) = args.name {
      let entry = config.passwords.get(&name);

      if entry.is_none() {
        eprintln!("no password named {}", name);
        return Ok(CommandResult {
          should_save: false,
          success: false,
        });
      }

      (name, entry.unwrap())
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

      let name = (*Vec::from_iter(config.passwords.keys())
        .choose(&mut thread_rng())
        .unwrap())
      .to_string();
      let entry = config.passwords.get(&name).unwrap();
      (name, entry)
    }
  };

  let mut tries = config.retries.unwrap_or_default() + 1;
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

  config.last_asked = Some(now);

  Ok(CommandResult {
    should_save: true,
    success,
  })
}
