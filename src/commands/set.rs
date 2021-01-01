use crate::config;
use crate::error::PasspromptError;
use std::convert::TryFrom;
use std::str::FromStr;

pub struct Args {
  pub key: String,
  pub value: String,
}

pub fn command(
  config: &mut config::Config,
  args: Args,
) -> Result<bool, Box<dyn std::error::Error>> {
  if args.key == "retries" {
    config.retries = Some(args.value.parse()?);
  } else if args.key == "wait" {
    config.wait = Some(args.value.parse()?);
  } else {
    return Err(Box::new(PasspromptError::UnknownConfigOption(args.key)));
  }

  Ok(true)
}

impl FromStr for config::Wait {
  type Err = Box<dyn std::error::Error>;

  fn from_str(s: &str) -> Result<config::Wait, Self::Err> {
    config::Wait::try_from(s.to_string())
  }
}
