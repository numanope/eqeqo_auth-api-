use httpageboy::{Request, Response, StatusCode};

fn text_response(status: StatusCode, body: &'static str) -> Response {
  Response {
    status: status.to_string(),
    content: body.as_bytes().to_vec(),
    content_type: "text/plain; charset=utf-8".to_string(),
  }
}

pub async fn list_users(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "List users")
}

pub async fn create_user(_req: &Request) -> Response {
  text_response(StatusCode::Created, "Create user")
}

pub async fn get_user(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Get user")
}

pub async fn update_user(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Update user")
}

pub async fn delete_user(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Delete user")
}