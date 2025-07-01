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

  pub fn update_user(&mut self, user: User) -> Result<(), String> {
    if let Some(index) = self.users.iter().position(|u| u.id == user.id) {
      self.users[index] = user;
      Ok(())
    } else {
      Err("User not found".to_string())
    }
  }

  pub fn delete_user(&mut self, id: &str) -> Result<(), String> {
    if let Some(index) = self.users.iter().position(|u| u.id == id) {
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
      id: "U1".to_string(),
      name: "John".to_string(),
    });
    self.add_user(User {
      id: "U2".to_string(),
      name: "Bob".to_string(),
    });
  }
}
