use httpageboy::{Request, Response, StatusCode};

pub fn list_users(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: String::new(),
    content: "Demo: Listar usuarios".as_bytes().to_vec(),
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
