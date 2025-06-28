use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct User {
  pub id: String,
  pub name: String,
  pub email: String,
}

pub struct DB {
  users: HashMap<String, User>,
}

impl DB {
  pub fn new() -> Self {
    DB {
      users: HashMap::new(),
    }
  }

  pub fn add_user(&mut self, user: User) {
    self.users.insert(user.id.clone(), user);
  }

  pub fn get_user(&self, id: &str) -> Option<&User> {
    self.users.get(id)
  }

  pub fn update_user(&mut self, user: User) -> Result<(), String> {
    if self.users.contains_key(&user.id) {
      self.users.insert(user.id.clone(), user);
      Ok(())
    } else {
      Err("User not found".to_string())
    }
  }

  pub fn delete_user(&mut self, id: &str) -> Result<(), String> {
    if self.users.contains_key(id) {
      self.users.remove(id);
      Ok(())
    } else {
      Err("User not found".to_string())
    }
  }

  pub fn list_users(&self) -> Vec<&User> {
    self.users.values().collect()
  }
}
