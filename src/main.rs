use base64;
use clap::{crate_version, App, Arg, SubCommand};
use crypto::bcrypt;
use rand::prelude::*;
use rpassword::prompt_password_stdout;
use rprompt::prompt_reply_stdout;
use xdg::BaseDirectories;

mod config;

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
    let mut config = config::Config::load_put_if_absent(&config_path)?;

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
            config::Password {
                salt,
                hash: base64::encode(hash),
            },
        );
    } else {
        println!("{}", matches.usage());
    }

    Ok(())
}
