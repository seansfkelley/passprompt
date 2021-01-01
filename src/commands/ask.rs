use rand::seq::SliceRandom;
use rand::thread_rng;
use rpassword::prompt_password_stdout;
use std::iter::FromIterator;
use std::time::SystemTime;

use crate::config;
use crate::error::PasspromptError;

pub struct Args {
  pub always: bool,
}

pub fn command(
  config: &mut config::Config,
  args: Args,
) -> Result<bool, Box<dyn std::error::Error>> {
  if config.passwords.len() == 0 {
    return Err(Box::new(PasspromptError::NoPasswordsDefined));
  }

  let now = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap()
    .as_secs();

  let should_ask = {
    if args.always {
      true
    } else {
      match config.last_asked {
        Some(timestamp) => {
          let wait_seconds = config.wait.unwrap_or_default().as_secs();
          timestamp + wait_seconds < now
        }
        None => true,
      }
    }
  };

  if !should_ask {
    return Ok(false);
  }

  let entries = Vec::from_iter(config.passwords.iter_mut());
  let entry = entries.choose(&mut thread_rng()).unwrap();

  let mut tries = config.retries.unwrap_or_default() + 1;
  let mut success = false;

  while tries > 0 {
    let input =
      prompt_password_stdout(format!("[passprompt] password for {}: ", entry.0).as_str())?;

    if input.len() > 0 && entry.1.matches(input) {
      success = true;
      break;
    }

    tries -= 1;
  }

  if success {
    println!("got it!");
  } else {
    println!("better luck next time...");
  }

  config.last_asked = Some(now);

  Ok(true)
}
