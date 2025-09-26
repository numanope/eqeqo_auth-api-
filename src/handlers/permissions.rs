use httpageboy::{Request, Response, StatusCode};

fn text_response(status: StatusCode, body: &'static str) -> Response {
  Response {
    status: status.to_string(),
    content: body.as_bytes().to_vec(),
    content_type: "text/plain; charset=utf-8".to_string(),
  }
}

pub async fn list_permissions(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "List permissions")
}

pub async fn create_permission(_req: &Request) -> Response {
  text_response(StatusCode::Created, "Create permission")
}

pub async fn get_permission(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Get permission")
}

pub async fn update_permission(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Update permission")
}

pub async fn delete_permission(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Delete permission")
}