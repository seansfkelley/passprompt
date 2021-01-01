use crate::error::PasspromptError;

pub fn byte_vec_to_array<const LEN: usize>(s: Vec<u8>) -> Result<[u8; LEN], PasspromptError> {
  if s.len() != LEN {
    return Err(PasspromptError::IllegalByteArrayLength {
      expected: LEN,
      actual: s.len(),
    });
  }
  let mut bytes_array = [0; LEN];
  for (i, b) in s.iter().enumerate() {
    bytes_array[i] = *b;
  }
  Ok(bytes_array)
}
