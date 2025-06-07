use base64;

use rand::prelude::*;
use rpassword::prompt_password_stderr;
use rprompt::prompt_reply_stderr;

use crate::commands::CommandResult;
use crate::config;

pub struct Args {
  pub name: Option<String>,
}

pub fn command(
  config: &mut config::Config,
  args: Args,
) -> Result<CommandResult, Box<dyn std::error::Error>> {
  let name = {
    if let Some(name) = args.name {
      name
    } else {
      prompt_reply_stderr("name: ")?
    }
  };

  if config.passwords.contains_key(&name) {
    let response = prompt_reply_stderr(
      format!(
        "there is already a password named '{}', overwrite (y/N)? ",
        name
      )
      .as_str(),
    )?;
    if response != "y" && response != "Y" {
      return Ok(CommandResult {
        save_config: false,
        success: true,
      });
    }
  }

  let salt = {
    let mut salt_bytes = [0; 16];
    rand::thread_rng().fill_bytes(&mut salt_bytes);
    base64::encode(salt_bytes)
  };

  let password = prompt_password_stderr("password: ")?;

  config
    .passwords
    .insert(name, config::PasswordEntry::create(salt, password)?);

  Ok(CommandResult {
    save_config: true,
    success: true,
  })
}
