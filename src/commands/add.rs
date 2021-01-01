use base64;

use rand::prelude::*;
use rpassword::prompt_password_stdout;
use rprompt::prompt_reply_stdout;

use crate::config;

pub struct Args {
  pub name: Option<String>,
}

pub fn command(
  config: &mut config::Config,
  args: Args,
) -> Result<bool, Box<dyn std::error::Error>> {
  let name = {
    if let Some(name) = args.name {
      name
    } else {
      prompt_reply_stdout("name: ")?
    }
  };

  if config.passwords.contains_key(&name) {
    let response = prompt_reply_stdout(
      format!(
        "there is already a password named '{}', overwrite (y/N)? ",
        name
      )
      .as_str(),
    )?;
    if response != "y" && response != "Y" {
      return Ok(false);
    }
  }

  let salt = {
    let mut salt_bytes = [0; 16];
    rand::thread_rng().fill_bytes(&mut salt_bytes);
    base64::encode(salt_bytes)
  };

  let password = prompt_password_stdout("password: ")?;

  config
    .passwords
    .insert(name, config::PasswordEntry::create(salt, password)?);

  Ok(true)
}
