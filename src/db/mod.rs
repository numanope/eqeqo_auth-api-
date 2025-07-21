use crate::classes::user::User;

pub struct DB {
  users: Vec<User>,
}

impl DB {
  pub fn new() -> Self {
    DB { users: vec![] }
  }

  pub fn add_user(&mut self, user: User) {
    self.users.push(user);
  }

  pub fn get_user(&self, id: &str) -> Option<&User> {
    self.users.iter().find(|u| u.id == id)
  }

  pub fn update_user(&mut self, user_id: String, updated_user: User) -> Result<(), String> {
    if let Some(index) = self.users.iter().position(|u| u.id == user_id) {
      self.users[index] = updated_user;
      Ok(())
    } else {
      Err("User not found".to_string())
    }
  }

  pub fn delete_user(&mut self, user_id: &str) -> Result<(), String> {
    if let Some(index) = self.users.iter().position(|u| u.id == user_id) {
      self.users.remove(index);
      Ok(())
    } else {
      Err("User not found".to_string())
    }
  }

  pub fn list_users(&self) -> Vec<&User> {
    self.users.iter().collect()
  }

  pub fn test_content(&mut self) {
    self.add_user(User {
      id: "U01".to_string(),
      name: "John".to_string(),
      hash: "hash1".to_string(),
      person_type: "natural".to_string(),
      document_type: "DNI".to_string(),
      document_number: "12345678".to_string(),
    });
    self.add_user(User {
      id: "U02".to_string(),
      name: "Bob".to_string(),
      hash: "hash2".to_string(),
      person_type: "juridical".to_string(),
      document_type: "RUC".to_string(),
      document_number: "20123456789".to_string(),
    });
    self.add_user(User {
      id: "U03".to_string(),
      name: "Jack".to_string(),
      hash: "hash3".to_string(),
      person_type: "natural".to_string(),
      document_type: "DNI".to_string(),
      document_number: "87654321".to_string(),
    });
    self.add_user(User {
      id: "U04".to_string(),
      name: "Alice".to_string(),
      hash: "hash4".to_string(),
      person_type: "natural".to_string(),
      document_type: "DNI".to_string(),
      document_number: "11223344".to_string(),
    });
    self.add_user(User {
      id: "U05".to_string(),
      name: "Eve".to_string(),
      hash: "hash5".to_string(),
      person_type: "juridical".to_string(),
      document_type: "RUC".to_string(),
      document_number: "20987654321".to_string(),
    });
    self.add_user(User {
      id: "U06".to_string(),
      name: "Charlie".to_string(),
      hash: "hash6".to_string(),
      person_type: "natural".to_string(),
      document_type: "DNI".to_string(),
      document_number: "44556677".to_string(),
    });
    self.add_user(User {
      id: "U07".to_string(),
      name: "David".to_string(),
      hash: "hash7".to_string(),
      person_type: "natural".to_string(),
      document_type: "DNI".to_string(),
      document_number: "99887766".to_string(),
    });
    self.add_user(User {
      id: "U08".to_string(),
      name: "Grace".to_string(),
      hash: "hash8".to_string(),
      person_type: "juridical".to_string(),
      document_type: "RUC".to_string(),
      document_number: "20775544332".to_string(),
    });
    self.add_user(User {
      id: "U09".to_string(),
      name: "Hank".to_string(),
      hash: "hash9".to_string(),
      person_type: "natural".to_string(),
      document_type: "DNI".to_string(),
      document_number: "33445566".to_string(),
    });
    self.add_user(User {
      id: "U10".to_string(),
      name: "Ivy".to_string(),
      hash: "hash10".to_string(),
      person_type: "natural".to_string(),
      document_type: "DNI".to_string(),
      document_number: "55667788".to_string(),
    });
    self.add_user(User {
      id: "U11".to_string(),
      name: "Oscar".to_string(),
      hash: "hash11".to_string(),
      person_type: "juridical".to_string(),
      document_type: "RUC".to_string(),
      document_number: "20442233119".to_string(),
    });
    self.add_user(User {
      id: "U12".to_string(),
      name: "Mia".to_string(),
      hash: "hash12".to_string(),
      person_type: "natural".to_string(),
      document_type: "DNI".to_string(),
      document_number: "66778899".to_string(),
    });
  }
}
