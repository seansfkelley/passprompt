#![feature(const_generics)]

use clap::{crate_version, App, Arg, SubCommand};
use xdg::BaseDirectories;

mod commands;
mod config;
mod error;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("passprompt")
        .version(crate_version!())
        .about("Nag yourself to remember your passwords.")
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .about("List known passwords"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new password")
                .arg(
                    Arg::with_name("name")
                        .value_name("NAME")
                        .help("Name for the new password"),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .alias("rm")
                .about("Remove one or more passwords")
                .arg(
                    Arg::with_name("password")
                        .value_name("PASSWORD")
                        .multiple(true)
                        .help("Name of the password to remove"),
                )
                .arg(
                    Arg::with_name("all")
                        .long("all")
                        .short("a")
                        .help("Remove all passwords"),
                ),
        )
        .subcommand(
            SubCommand::with_name("ask")
                .about("Prompt for a random password from the list")
                .arg(
                    Arg::with_name("always")
                        .long("always")
                        .short("a")
                        .help("Always ask, even if the configured wait time hasn't elapsed"),
                ),
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Sets or gets a configuration value (see documentation)")
                .arg(
                    Arg::with_name("key")
                        .value_name("KEY")
                        .required(true)
                        .help("Name of the configuration option to set"),
                )
                .arg(
                    Arg::with_name("value")
                        .value_name("VALUE")
                        .help("Value to assign the configuration option"),
                ),
        )
        .get_matches();

    // TODO: Is the xdg library necessary?
    let xdg_dirs = BaseDirectories::with_prefix("passprompt").unwrap();
    let config_path = xdg_dirs.place_config_file("config.toml")?;
    let mut config = config::Config::load(&config_path)?;

    let result = {
        if let Some(_) = matches.subcommand_matches("list") {
            commands::list(&config)?
        } else if let Some(matches) = matches.subcommand_matches("config") {
            commands::config(
                &mut config,
                commands::ConfigArgs {
                    key: matches.value_of("key").unwrap().to_string(),
                    value: matches.value_of("value").map(|v| v.to_string()),
                },
            )?
        } else if let Some(matches) = matches.subcommand_matches("add") {
            commands::add(
                &mut config,
                commands::AddArgs {
                    name: matches.value_of("name").map(|n| n.to_string()),
                },
            )?
        } else if let Some(matches) = matches.subcommand_matches("remove") {
            commands::remove(
                &mut config,
                commands::RemoveArgs {
                    all: matches.is_present("all"),
                    entries: matches
                        .values_of("password")
                        .map(|v| v.into_iter().collect())
                        .unwrap_or(vec![]),
                },
            )?
        } else if let Some(matches) = matches.subcommand_matches("ask") {
            commands::ask(
                &mut config,
                commands::AskArgs {
                    always: matches.is_present("always"),
                },
            )?
        } else {
            eprintln!("{}", matches.usage());
            commands::CommandResult {
                should_save: false,
                success: true,
            }
        }
    };

    if result.should_save {
        config.store(&config_path)?;
    }

    if !result.success {
        std::process::exit(1);
    } else {
        Ok(())
    }
}
