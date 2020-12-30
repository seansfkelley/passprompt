use base64;
use clap::{crate_version, App, Arg, SubCommand};
use crypto::bcrypt;
use rand::prelude::*;
use rpassword::prompt_password_stdout;
use rprompt::prompt_reply_stdout;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{write, OpenOptions};
use std::io::Read;
use toml;
use xdg::BaseDirectories;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    frequency: Option<String>,
    hash: Option<String>,
    passwords: HashMap<String, Password>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Password {
    salt: String,
    hash: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("passprompt")
        .version(crate_version!())
        .about("Occasionally prompt for passwords as a memorization aid.")
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .about("List known passwords"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new password entry")
                .arg(
                    Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .takes_value(true)
                        .help("Name for the new password"),
                )
                .arg(
                    Arg::with_name("salt")
                        .long("salt")
                        .short("s")
                        .takes_value(true)
                        .help("Salt to use for hashing"),
                ),
        )
        .get_matches();

    let xdg_dirs = BaseDirectories::with_prefix("passprompt").unwrap();
    let config_path = xdg_dirs.place_config_file("config.toml")?;
    let mut config_content = String::new();

    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(config_path.clone())?
        .read_to_string(&mut config_content)?;

    let mut config: Config = toml::from_str(&config_content.to_string())?;

    if let Some(_) = matches.subcommand_matches("list") {
        // TODO: Should not need to unwrap because this list should always be present.
        for (name, _) in config.passwords.into_iter() {
            println!("{}", name);
        }
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let name = {
            if let Some(name) = matches.value_of("name") {
                name.to_string()
            } else {
                prompt_reply_stdout("name: ")?
            }
        };
        let salt = {
            if let Some(salt) = matches.value_of("salt") {
                salt.to_string()
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
            Password {
                salt,
                hash: base64::encode(hash),
            },
        );

        write(config_path, toml::to_string_pretty(&config)?)?;
    } else {
        println!("{}", matches.usage());
    }

    Ok(())
}
