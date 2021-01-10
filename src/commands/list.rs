use std::iter::FromIterator;

use crate::commands::CommandResult;
use crate::config;

pub fn command(config: &config::Config) -> Result<CommandResult, Box<dyn std::error::Error>> {
  let mut names = Vec::from_iter(config.passwords.keys());
  names.sort_unstable();

  for n in names {
    println!("{}", n);
  }

  Ok(CommandResult {
    should_save: false,
    success: true,
  })
}
