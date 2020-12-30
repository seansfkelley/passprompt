use clap::{crate_version, App, Arg, SubCommand};
use xdg::BaseDirectories;

mod commands;
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
        commands::list(&mut config)?;
    } else if let Some(matches) = matches.subcommand_matches("add") {
        commands::add(
            &mut config,
            commands::AddArgs {
                name: matches.value_of("name").map(|n| n.to_string()),
                salt: matches.value_of("salt").map(|s| s.to_string()),
            },
        )?;
    } else {
        println!("{}", matches.usage());
    }

    config.store(&config_path)?;

    Ok(())
}
