use std::error;
use std::fmt;

#[derive(Debug)]
pub enum PasspromptError {
  NoPasswordsDefined,
  ImproperSaltLength(usize, usize),
  UnparseableWaitFormat(String),
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
      Self::UnparseableWaitFormat(s) => {
        write!(f, "could not parse wait format '{}'", s)
      }
    }
  }
}

impl error::Error for PasspromptError {}
