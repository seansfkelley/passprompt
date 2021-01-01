use crate::config;

pub fn command(config: &config::Config) -> Result<bool, Box<dyn std::error::Error>> {
  for (name, _) in config.passwords.iter() {
    println!("{}", name);
  }

  Ok(true)
}
