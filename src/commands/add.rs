use base64;

use rand::prelude::*;
use rpassword::prompt_password_stdout;
use rprompt::prompt_reply_stdout;

use crate::config;

pub struct Args {
  pub name: Option<String>,
  pub salt: Option<String>,
}

pub fn command(config: &mut config::Config, args: Args) -> Result<(), Box<dyn std::error::Error>> {
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
        "there is already a password named '{}', overwrite (y/n)? ",
        name
      )
      .as_str(),
    )?;
    if response != "y" && response != "Y" {
      return Ok(());
    }
  }

  let salt = {
    if let Some(salt) = args.salt {
      base64::decode(&salt)?;
      salt
    } else {
      let mut salt = [0; 16];
      rand::thread_rng().fill_bytes(&mut salt);
      base64::encode(salt)
    }
  };

  let password = prompt_password_stdout("password: ")?;

  config
    .passwords
    .insert(name, config::PasswordEntry::create(salt, password)?);

  Ok(())
}
