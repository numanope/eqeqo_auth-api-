use std::fmt;

#[derive(Debug, Clone, serde::Serialize)]
pub struct User {
  pub country: u8,
  pub id: String,
  pub name: String,
  pub lastname: String,
  pub hash: String,
}

impl fmt::Display for User {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ID: {}, Name: {}", self.id, self.name)
  }
}

impl User {
  pub fn new(id: String, name: String) -> Self {
    User { id, name }
  }
}
