use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: String,
  pub name: String,
  pub hash: String,
  pub person_type: String,
  pub document_type: String,
  pub document_number: String,
}

impl fmt::Display for User {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ID: {}, Name: {}", self.id, self.name)
  }
}

impl User {
  pub fn new(
    id: String,
    hash: String,
    name: String,
    person_type: String,
    document_type: String,
    document_number: String,
  ) -> Self {
    User {
      id,
      hash,
      name: if name.is_empty() { String::new() } else { name },
      person_type,
      document_type,
      document_number,
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn get_person_type(&self) -> &String {
    &self.person_type
  }

  pub fn set_person_type(&mut self, person_type: String) {
    self.person_type = person_type;
  }

  pub fn get_document_type(&self) -> &String {
    &self.document_type
  }

  pub fn set_document_type(&mut self, document_type: String) {
    self.document_type = document_type;
  }

  pub fn get_document_number(&self) -> &String {
    &self.document_number
  }

  pub fn set_document_number(&mut self, document_number: String) {
    self.document_number = document_number;
  }
}
