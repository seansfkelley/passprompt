pub fn byte_vec_to_array<const LEN: usize>(
  s: Vec<u8>,
) -> Result<[u8; LEN], Box<dyn std::error::Error>> {
  if s.len() != LEN {
    panic!("todo");
  }
  let mut bytes_array = [0; LEN];
  for (i, b) in s.iter().enumerate() {
    bytes_array[i] = *b;
  }
  Ok(bytes_array)
}
