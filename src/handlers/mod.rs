use crate::{classes::user::User, db::DB};
use httpageboy::{Request, Response, StatusCode};
use serde_json::json;

pub fn list_users(_req: &Request) -> Response {
  let mut db = DB::new();
  db.test_content();
  let users: Vec<&User> = db.list_users();
  let json_response = json!(users).to_string();
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "application/json".to_string(),
    content: json_response.as_bytes().to_vec(),
  }
}

pub fn get_user(_req: &Request) -> Response {
  let mut db = DB::new();
  db.test_content();
  let user = db.get_user(_req.params.get("id").unwrap());
  let json_response = json!(user).to_string();

  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "application/json".to_string(),
    content: json_response.as_bytes().to_vec(),
  }
}

// TODO
pub fn create_user(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Crear usuario".as_bytes().to_vec(),
  }
}

// TODO
pub fn update_user(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Editar usuario".as_bytes().to_vec(),
  }
}

// TODO
pub fn delete_user(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: Vec::new(),
  }
}

// TODO
pub fn list_roles(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Listar roles".as_bytes().to_vec(),
  }
}

// TODO
pub fn create_role(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Crear rol".as_bytes().to_vec(),
  }
}

// TODO
pub fn delete_role(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Eliminar rol".as_bytes().to_vec(),
  }
}
