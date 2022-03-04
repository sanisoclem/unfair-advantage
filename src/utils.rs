use sha2::{Digest, Sha256};

pub fn create_hash(text: &str) -> String {
  let mut hasher = Sha256::default();
  hasher.update(text.as_bytes());
  format!("{:x}", hasher.finalize())
}
