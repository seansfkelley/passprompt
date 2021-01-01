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
  let mut entries = Vec::from_iter(config.passwords.iter_mut());

  if !args.always {
    let since = config.wait.unwrap_or_default().to_millis();
    entries = entries
      .into_iter()
      .filter(|e| e.1.last_asked.unwrap_or(0) < since)
      .collect();
  } else if entries.len() == 0 {
    return Err(Box::new(PasspromptError::NoPasswordsDefined));
  }

  if entries.len() == 0 {
    return Ok(false);
  }

  let entry = entries.choose_mut(&mut thread_rng()).unwrap();

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

  entry.1.last_asked = Some(
    SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_millis() as u64,
  );

  Ok(true)
}
