#![feature(const_generics)]

use clap::{crate_version, App, Arg, SubCommand};
use xdg::BaseDirectories;

mod commands;
mod config;
mod util;

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
                        .help("16-byte salt to use for hashing, encoded in base64"),
                ),
        )
        .subcommand(
            SubCommand::with_name("ask").about("Prompt for a random password from the list"),
        )
        .get_matches();

    let xdg_dirs = BaseDirectories::with_prefix("passprompt").unwrap();
    let config_path = xdg_dirs.place_config_file("config.toml")?;
    let mut config = config::Config::load(&config_path)?;

    if let Some(_) = matches.subcommand_matches("list") {
        commands::list(&config)?;
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let did_update = commands::add(
            &mut config,
            commands::AddArgs {
                name: matches.value_of("name").map(|n| n.to_string()),
                salt: matches.value_of("salt").map(|s| s.to_string()),
            },
        )?;
        if did_update {
            config.store(&config_path)?;
        }
    } else if let Some(matches) = matches.subcommand_matches("ask") {
        commands::ask(&config)?;
    } else {
        println!("{}", matches.usage());
    }

    Ok(())
}
