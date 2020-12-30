use crate::config;

pub fn command(config: &mut config::Config) -> Result<(), Box<dyn std::error::Error>> {
  for (name, _) in config.passwords.iter() {
    println!("{}", name);
  }

  Ok(())
}
