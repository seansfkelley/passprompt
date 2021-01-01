use crate::commands::CommandResult;
use crate::config;

pub fn command(config: &config::Config) -> Result<CommandResult, Box<dyn std::error::Error>> {
  for (name, _) in config.passwords.iter() {
    println!("{}", name);
  }

  Ok(CommandResult {
    should_save: false,
    success: true,
  })
}
