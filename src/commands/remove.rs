use crate::commands::CommandResult;
use crate::config;

pub struct Args<'a> {
  pub all: bool,
  pub entries: Vec<&'a str>,
}

pub fn command(
  config: &mut config::Config,
  args: Args,
) -> Result<CommandResult, Box<dyn std::error::Error>> {
  if args.all {
    config.passwords = Default::default();
  } else {
    for e in args.entries.iter() {
      config.passwords.remove(&e.to_string());
    }
  }

  Ok(CommandResult {
    should_save: true,
    success: true,
  })
}
