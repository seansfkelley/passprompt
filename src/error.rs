use std::error;
use std::fmt;

#[derive(Debug)]
pub enum PasspromptError {
  NoPasswordsDefined,
  IllegalByteArrayLength { expected: usize, actual: usize },
  UnparseableWaitFormat(String),
  UnknownConfigOption(String),
}

impl fmt::Display for PasspromptError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::IllegalByteArrayLength { expected, actual } => {
        write!(f, "improper length: expected {}, got {}", expected, actual)
      }
      Self::NoPasswordsDefined => {
        write!(f, "no passwords defined")
      }
      Self::UnparseableWaitFormat(s) => {
        write!(f, "could not parse wait format '{}'", s)
      }
      Self::UnknownConfigOption(o) => {
        write!(f, "unknown configuration option '{}'", o)
      }
    }
  }
}

impl error::Error for PasspromptError {}
