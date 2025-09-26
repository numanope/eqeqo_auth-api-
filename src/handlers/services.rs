use httpageboy::{Request, Response, StatusCode};

fn text_response(status: StatusCode, body: &'static str) -> Response {
  Response {
    status: status.to_string(),
    content: body.as_bytes().to_vec(),
    content_type: "text/plain; charset=utf-8".to_string(),
  }
}

pub async fn list_services(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "List services")
}

pub async fn create_service(_req: &Request) -> Response {
  text_response(StatusCode::Created, "Create service")
}

pub async fn get_service(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Get service")
}

pub async fn update_service(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Update service")
}

pub async fn delete_service(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Delete service")
}