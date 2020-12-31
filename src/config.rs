use crypto::bcrypt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{write, OpenOptions};
use std::io::Read;
use std::path::PathBuf;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub frequency: Option<String>,
    pub hash: Option<String>,
    pub passwords: HashMap<String, Password>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Password {
    pub salt: String,
    pub hash: String,
}

impl Config {
    pub fn load_put_if_absent(p: &PathBuf) -> Result<Config, Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(p.clone());

        if let Err(f) = file {
            let mut config_content = String::new();
            OpenOptions::new()
                .read(true)
                .open(p.clone())?
                .read_to_string(&mut config_content)?;
            if let Ok(c) = toml::from_str::<Config>(&config_content.to_string()) {
                Ok(c)
            } else {
                panic!("todo");
            }
        } else {
            let c = Config {
                frequency: None,
                hash: None,
                passwords: HashMap::new(),
            };
            c.store(&p)?;
            Ok(c)
        }
    }

    pub fn store(&self, p: &PathBuf) -> Result<(), std::io::Error> {
        if let Ok(foo) = toml::to_string_pretty(&self) {
            write(p, foo)
        } else {
            panic!("todo");
        }
    }
}

impl Password {
    pub fn create(salt: String, password: String) -> Password {
        Password {
            hash: Password::hash(salt.clone(), password),
            salt,
        }
    }

    pub fn matches(&self, password: String) -> bool {
        self.hash == Password::hash(self.salt.clone(), password)
    }

    fn hash(salt: String, password: String) -> String {
        let mut hash = [0; 24];

        bcrypt::bcrypt(
            12, // 12 is the work factor recommended by OWASP.
            &salt.into_bytes(),
            &password.into_bytes(),
            &mut hash,
        );

        base64::encode(hash)
    }
}
