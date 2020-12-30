use base64;
use crypto::bcrypt;
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
  let salt = {
    if let Some(salt) = args.salt {
      salt
    } else {
      // OWASP says salts should be > 16 characters; after base64ing, 12 characters
      // becomes 16.
      let mut salt = [0; 12];
      rand::thread_rng().fill_bytes(&mut salt);
      base64::encode(salt)
    }
  };
  let password = prompt_password_stdout("password: ")?;
  let mut hash = [0; 24];

  bcrypt::bcrypt(
    12, // 12 is the work factor recommended by OWASP.
    &salt.clone().into_bytes(),
    &password.into_bytes(),
    &mut hash,
  );

  // TODO: Make sure this isn't a dupe and/or warn about it.
  config.passwords.insert(
    name,
    config::Password {
      salt,
      hash: base64::encode(hash),
    },
  );

  Ok(())
}
