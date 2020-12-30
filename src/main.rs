use clap::{crate_version, App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use std::fs::{read_dir, File, OpenOptions};
use std::io::Read;
use toml;
use xdg::BaseDirectories;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    frequency: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("passprompt")
        .version(crate_version!())
        .about("Occasionally prompt for passwords as a memorization aid.")
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .help("List known passwords"),
        )
        .get_matches();

    let xdg_dirs = BaseDirectories::with_prefix("passprompt").unwrap();
    let config_path = xdg_dirs.place_config_file("config.toml")?;
    let passwords_path = xdg_dirs.create_config_directory("passwords")?;
    let mut config_content = String::new();

    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(config_path)?
        .read_to_string(&mut config_content)?;

    let config: Config = toml::from_str(&config_content.to_string())?;

    if let Some(_) = matches.subcommand_matches("list") {
        // TODO: Perhaps lighten up some of this ? usage to be more fault-tolerant.
        for entry in read_dir(passwords_path)? {
            let path = entry?;
            if path.file_type()?.is_file() {
                println!("{}", path.file_name().to_str().unwrap());
            }
        }
    }

    Ok(())
}
