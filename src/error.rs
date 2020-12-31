use std::error;
use std::fmt;

#[derive(Debug)]
pub enum PasspromptError {
  NoPasswordsDefined,
  ImproperSaltLength(usize, usize),
}

impl fmt::Display for PasspromptError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ImproperSaltLength(expected, actual) => {
        write!(f, "improper length: expected {}, got {}", expected, actual)
      }
      Self::NoPasswordsDefined => {
        write!(f, "no passwords defined")
      }
    }
  }
}

impl error::Error for PasspromptError {}
