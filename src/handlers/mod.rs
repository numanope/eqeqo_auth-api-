use crate::db::DB;
use httpageboy::{Request, Response, StatusCode};

pub fn list_users(_req: &Request, db: &DB) -> Response {
  let users = db.list_users();
  let user_list: Vec<String> = users
    .iter()
    .map(|user| {
      format!(
        "ID: {}, Name: {}, Email: {}",
        user.id, user.name, user.email
      )
    })
    .collect();
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: user_list.join("\n").as_bytes().to_vec(),
  }
}

pub fn get_user(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Ver perfil de usuario".as_bytes().to_vec(),
  }
}

pub fn create_user(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Crear usuario".as_bytes().to_vec(),
  }
}

pub fn update_user(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Editar usuario".as_bytes().to_vec(),
  }
}

pub fn delete_user(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: Vec::new(),
  }
}

pub fn list_roles(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Listar roles".as_bytes().to_vec(),
  }
}

pub fn create_role(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Crear rol".as_bytes().to_vec(),
  }
}

pub fn delete_role(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Eliminar rol".as_bytes().to_vec(),
  }
}
