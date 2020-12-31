use rand::seq::SliceRandom;
use rand::thread_rng;
use rpassword::prompt_password_stdout;
use std::iter::FromIterator;
use std::time::SystemTime;

use crate::config;
use crate::error::PasspromptError;

pub fn command(config: &mut config::Config) -> Result<bool, Box<dyn std::error::Error>> {
  let mut entries = Vec::from_iter(config.passwords.iter_mut());

  if entries.len() == 0 {
    return Err(Box::new(PasspromptError::NoPasswordsDefined));
  }

  let entry = entries.choose_mut(&mut thread_rng()).unwrap();

  let mut tries = config.retries + 1;
  let mut success = false;

  while tries > 0 {
    let input = prompt_password_stdout(format!("password for {}: ", entry.0).as_str())?;

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
