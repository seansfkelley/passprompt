use std::error;
use std::fmt;

#[derive(Debug)]
struct ImproperLengthError {
  expected: usize,
  actual: usize,
}

impl fmt::Display for ImproperLengthError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "improper length: expected {}, got {}",
      self.expected, self.actual
    )
  }
}

impl error::Error for ImproperLengthError {}

pub fn byte_vec_to_array<const LEN: usize>(s: Vec<u8>) -> Result<[u8; LEN], Box<dyn error::Error>> {
  if s.len() != LEN {
    return Err(Box::new(ImproperLengthError {
      expected: LEN,
      actual: s.len(),
    }));
  }
  let mut bytes_array = [0; LEN];
  for (i, b) in s.iter().enumerate() {
    bytes_array[i] = *b;
  }
  Ok(bytes_array)
}
