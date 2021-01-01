use crypto::bcrypt;
use regex::{Match, Regex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fs::{write, OpenOptions};
use std::io::Read;
use std::path::PathBuf;
use toml;

use crate::error::PasspromptError;
use crate::util;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub wait: Option<Wait>,
    pub retries: Option<usize>,
    pub last_asked: Option<u64>,
    #[serde(default)]
    pub passwords: HashMap<String, PasswordEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct Wait {
    days: u64,
    hours: u64,
    minutes: u64,
}

impl Wait {
    fn parse_maybe_int(s: Option<Match>) -> u64 {
        s.map(|t| t.as_str().parse().unwrap()).unwrap_or(0)
    }

    pub fn as_secs(&self) -> u64 {
        ((self.days * 24 + self.hours) * 60 + self.minutes) * 60
    }
}

impl Default for Wait {
    fn default() -> Wait {
        Wait {
            days: 0,
            hours: 0,
            minutes: 0,
        }
    }
}

impl TryFrom<String> for Wait {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let parts = Regex::new(r"^\s*((?P<d>\d+)d)?\s*((?P<h>\d+)h)?\s*((?P<m>\d+)m)?\s*$")
            .unwrap()
            .captures(s.as_str());
        match parts {
            Some(captures) => Ok(Wait {
                days: Wait::parse_maybe_int(captures.name("d")),
                hours: Wait::parse_maybe_int(captures.name("h")),
                minutes: Wait::parse_maybe_int(captures.name("m")),
            }),
            None => Err(Box::new(PasspromptError::UnparseableWaitFormat(s))),
        }
    }
}

impl From<Wait> for String {
    fn from(w: Wait) -> Self {
        let mut s = String::new();
        if w.days > 0 {
            s += format!("{}d", w.days).as_str();
        }
        if w.hours > 0 {
            if s.len() > 0 {
                s += " ";
            }
            s += format!("{}h", w.hours).as_str();
        }
        if w.minutes > 0 {
            if s.len() > 0 {
                s += " ";
            }
            s += format!("{}m", w.minutes).as_str();
        }
        s
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntry {
    pub salt: Salt,
    pub hash: Hash,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct Salt([u8; 16]);

impl TryFrom<String> for Salt {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let bytes = base64::decode(s.as_str())?;
        Ok(Salt(util::byte_vec_to_array(bytes)?))
    }
}

impl From<Salt> for String {
    fn from(s: Salt) -> Self {
        base64::encode(&s.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct Hash([u8; 24]);

impl TryFrom<String> for Hash {
    type Error = Box<dyn std::error::Error>;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let bytes = base64::decode(s.as_str())?;
        Ok(Hash(util::byte_vec_to_array(bytes)?))
    }
}

impl From<Hash> for String {
    fn from(s: Hash) -> Self {
        base64::encode(&s.0)
    }
}

impl Config {
    pub fn load(p: &PathBuf) -> Result<Config, Box<dyn Error>> {
        let mut config_content = String::new();

        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&p)?
            .read_to_string(&mut config_content)?;

        match toml::from_str::<Config>(&config_content.to_string()) {
            Ok(c) => Ok(c),
            // TODO: Literally no idea why Box::new works here, but using map_err on the Result
            // directly causes it to complain about struct/trait object mismatch.
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn store(&self, p: &PathBuf) -> Result<(), Box<dyn Error>> {
        match toml::to_string_pretty(&self) {
            Ok(contents) => match write(p, contents) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl PasswordEntry {
    pub fn create(
        salt: String,
        password: String,
    ) -> Result<PasswordEntry, Box<dyn std::error::Error>> {
        let salt = Salt::try_from(salt)?;
        let hash = PasswordEntry::hash(&salt, password)?;
        Ok(PasswordEntry { salt, hash })
    }

    pub fn matches(&self, password: String) -> bool {
        if let Ok(h) = PasswordEntry::hash(&self.salt, password) {
            self.hash.0 == h.0
        } else {
            false
        }
    }

    fn hash(salt: &Salt, password: String) -> Result<Hash, Box<dyn std::error::Error>> {
        let mut hash = [0; 24];

        bcrypt::bcrypt(
            12, // 12 is the work factor recommended by OWASP.
            &salt.0,
            &password.into_bytes(),
            &mut hash,
        );

        Ok(Hash::try_from(base64::encode(hash))?)
    }
}
