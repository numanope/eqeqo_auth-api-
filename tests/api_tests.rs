#[cfg(test)]
mod tests {
  use httpageboy::test_utils::{run_test, setup_test_server, POOL_SIZE, SERVER_URL};
  use httpageboy::{Request, Response, Rt, Server, StatusCode};

  fn create_test_server() -> Server {
    let mut server = Server::new(SERVER_URL, POOL_SIZE, None).unwrap();

    server.add_route("/users/", Rt::GET, list_users);
    server.add_route("/users/{id}", Rt::GET, get_user);
    server.add_route("/users/", Rt::POST, create_user);
    server.add_route("/users/{id}", Rt::PUT, update_user);
    server.add_route("/users/{id}", Rt::DELETE, delete_user);
    server.add_route("/roles/", Rt::GET, list_roles);
    server.add_route("/roles/", Rt::POST, create_role);

    server
  }

  #[test]
  fn test_list_users() {
    setup_test_server(|| create_test_server());
    let request = b"GET /users/ HTTP/1.1\r\n\r\n\r\n";
    let expected_fragment = b"Demo: Listar usuarios";
    run_test(request, expected_fragment);
  }

  #[test]
  fn test_get_user() {
    setup_test_server(|| create_test_server());
    let request = b"GET /users/{id} HTTP/1.1\r\n\r\n\r\n";
    let expected_fragment = b"Demo: Ver perfil de usuario";
    run_test(request, expected_fragment);
  }

  #[test]
  fn test_create_user() {
    setup_test_server(|| create_test_server());
    let request = b"POST /users/ HTTP/1.1\r\n\r\n\r\n";
    let expected_fragment = b"Demo: Crear usuario";
    run_test(request, expected_fragment);
  }

  #[test]
  fn test_update_user() {
    setup_test_server(|| create_test_server());
    let request = b"PUT /users/{id} HTTP/1.1\r\n\r\n\r\n";
    let expected_fragment = b"Demo: Editar usuario";
    run_test(request, expected_fragment);
  }

  #[test]
  fn test_delete_user() {
    setup_test_server(|| create_test_server());
    let request = b"DELETE /users/{id} HTTP/1.1\r\n\r\n\r\n";
    let expected_fragment = b"Demo: Eliminar usuario";
    run_test(request, expected_fragment);
  }

  #[test]
  fn test_list_roles() {
    setup_test_server(|| create_test_server());
    let request = b"GET /roles/ HTTP/1.1\r\n\r\n\r\n";
    let expected_fragment = b"Demo: Listar roles";
    run_test(request, expected_fragment);
  }

  #[test]
  fn test_create_role() {
    setup_test_server(|| create_test_server());
    let request = b"POST /roles/ HTTP/1.1\r\n\r\n\r\n";
    let expected_fragment = b"Demo: Crear rol";
    run_test(request, expected_fragment);
  }

  fn list_users(_req: &Request) -> Response {
    Response {
      status: StatusCode::Ok.to_string(),
      content_type: String::new(),
      content: "Demo: Listar usuarios".as_bytes().to_vec(),
    }
  }

  fn get_user(_req: &Request) -> Response {
    Response {
      status: StatusCode::Ok.to_string(),
      content_type: String::new(),
      content: "Demo: Ver perfil de usuario".as_bytes().to_vec(),
    }
  }

  fn create_user(_req: &Request) -> Response {
    Response {
      status: StatusCode::Ok.to_string(),
      content_type: String::new(),
      content: "Demo: Crear usuario".as_bytes().to_vec(),
    }
  }

  fn update_user(_req: &Request) -> Response {
    Response {
      status: StatusCode::Ok.to_string(),
      content_type: String::new(),
      content: "Demo: Editar usuario".as_bytes().to_vec(),
    }
  }

  fn delete_user(_req: &Request) -> Response {
    Response {
      status: StatusCode::Ok.to_string(),
      content_type: String::new(),
      content: "Demo: Eliminar usuario".as_bytes().to_vec(),
    }
  }

  fn list_roles(_req: &Request) -> Response {
    Response {
      status: StatusCode::Ok.to_string(),
      content_type: String::new(),
      content: "Demo: Listar roles".as_bytes().to_vec(),
    }
  }

  fn create_role(_req: &Request) -> Response {
    Response {
      status: StatusCode::Ok.to_string(),
      content_type: String::new(),
      content: "Demo: Crear rol".as_bytes().to_vec(),
    }
  }
}
