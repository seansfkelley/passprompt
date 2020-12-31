use rand::seq::SliceRandom;
use rand::thread_rng;
use rpassword::prompt_password_stdout;
use std::iter::FromIterator;

use crate::config;

pub fn command(config: &config::Config) -> Result<(), Box<dyn std::error::Error>> {
  let entries = Vec::from_iter(config.passwords.iter());

  // TODO: Explode with a useful message if there are no entries.
  let entry = entries.choose(&mut thread_rng()).unwrap();

  let input = prompt_password_stdout(format!("password for {}: ", entry.0).as_str())?;

  if entry.1.matches(input) {
    println!("got it!");
  } else {
    println!("better luck next time");
  }

  Ok(())
}
