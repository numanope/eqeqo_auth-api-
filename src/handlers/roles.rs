use httpageboy::{Request, Response, StatusCode};

fn text_response(status: StatusCode, body: &'static str) -> Response {
  Response {
    status: status.to_string(),
    content: body.as_bytes().to_vec(),
    content_type: "text/plain; charset=utf-8".to_string(),
  }
}

pub async fn list_roles(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "List roles")
}

pub async fn create_role(_req: &Request) -> Response {
  text_response(StatusCode::Created, "Create role")
}

pub async fn get_role(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Get role")
}

pub async fn update_role(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Update role")
}

pub async fn delete_role(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Delete role")
}