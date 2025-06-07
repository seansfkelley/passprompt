use std::convert::TryFrom;
use std::str::FromStr;

use crate::commands::CommandResult;
use crate::config;
use crate::error::PasspromptError;

pub struct Args {
  pub key: String,
  pub value: Option<String>,
}

pub fn command(
  config: &mut config::Config,
  args: Args,
) -> Result<CommandResult, Box<dyn std::error::Error>> {
  if args.key == "retries" {
    if let Some(v) = args.value {
      config.retries = Some(v.parse()?);
    } else {
      println!("{}", config.retries.unwrap_or_default());
    }
  } else if args.key == "wait" {
    if let Some(v) = args.value {
      config.wait = Some(v.parse()?);
    } else {
      println!("{}", config.wait.unwrap_or_default());
    }
  } else {
    return Err(Box::new(PasspromptError::UnknownConfigOption(args.key)));
  }

  Ok(CommandResult {
    save_config: true,
    success: true,
  })
}

impl FromStr for config::Wait {
  type Err = Box<dyn std::error::Error>;

  fn from_str(s: &str) -> Result<config::Wait, Self::Err> {
    config::Wait::try_from(s.to_string())
  }
}
