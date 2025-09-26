use httpageboy::{Request, Response, StatusCode};

fn text_response(status: StatusCode, body: &'static str) -> Response {
  Response {
    status: status.to_string(),
    content: body.as_bytes().to_vec(),
    content_type: "text/plain; charset=utf-8".to_string(),
  }
}

// Role-Permissions
pub async fn assign_permission_to_role(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Assign permission to role")
}

pub async fn remove_permission_from_role(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Remove permission from role")
}

pub async fn list_role_permissions(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "List role permissions")
}

// Service-Roles
pub async fn assign_role_to_service(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Assign role to service")
}

pub async fn remove_role_from_service(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Remove role from service")
}

pub async fn list_service_roles(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "List service roles")
}

// Person-Service-Roles
pub async fn assign_role_to_person_in_service(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Assign role to person in service")
}

pub async fn remove_role_from_person_in_service(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Remove role from person in service")
}

pub async fn list_person_roles_in_service(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "List person roles in service")
}

pub async fn list_persons_with_role_in_service(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "List persons with role in service")
}

// Other checks
pub async fn check_person_permission_in_service(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "Check person permission in service")
}

pub async fn list_services_of_person(_req: &Request) -> Response {
  text_response(StatusCode::Ok, "List services of person")
}