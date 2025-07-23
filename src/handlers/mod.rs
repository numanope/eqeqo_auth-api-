use crate::classes::user::{Permission, Role, Service, User};
use crate::db::DB;
use httpageboy::{Request, Response, StatusCode};
use serde_json::json;
mod permissions;
mod relations;
mod roles;
mod services;
mod users;

pub fn home(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/html".to_string(),
    content: "<h1>Welcome to the Auth API</h1>".as_bytes().to_vec(),
  }
}

// Users
pub fn list_users(_req: &Request) -> Response {
  let mut db = DB::new();
  let users = db.list_users();
  let json_response = json!(users).to_string();
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "application/json".to_string(),
    content: json_response.as_bytes().to_vec(),
  }
}

pub fn get_user(_req: &Request) -> Response {
  let mut db = DB::new();
  let user = db.get_user(_req.params.get("id").unwrap());
  let json_response = json!(user).to_string();
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "application/json".to_string(),
    content: json_response.as_bytes().to_vec(),
  }
}

pub fn create_user(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Create user".as_bytes().to_vec(),
  }
}

pub fn update_user(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Update user".as_bytes().to_vec(),
  }
}

pub fn delete_user(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Delete user".as_bytes().to_vec(),
  }
}

// Services
pub fn list_services(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "List services".as_bytes().to_vec(),
  }
}

pub fn create_service(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Create service".as_bytes().to_vec(),
  }
}

pub fn update_service(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Update service".as_bytes().to_vec(),
  }
}

pub fn delete_service(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Delete service".as_bytes().to_vec(),
  }
}

// Roles
pub fn list_roles(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "List roles".as_bytes().to_vec(),
  }
}

pub fn get_role(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Get role".as_bytes().to_vec(),
  }
}

pub fn create_role(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Create role".as_bytes().to_vec(),
  }
}

pub fn update_role(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Update role".as_bytes().to_vec(),
  }
}

pub fn delete_role(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Delete role".as_bytes().to_vec(),
  }
}

// Permissions
pub fn list_permissions(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "List permissions".as_bytes().to_vec(),
  }
}

pub fn create_permission(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Create permission".as_bytes().to_vec(),
  }
}

pub fn update_permission(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Update permission".as_bytes().to_vec(),
  }
}

pub fn delete_permission(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Delete permission".as_bytes().to_vec(),
  }
}

// Role-Permissions
pub fn assign_permission_to_role(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Assign permission to role".as_bytes().to_vec(),
  }
}

pub fn remove_permission_from_role(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Remove permission from role".as_bytes().to_vec(),
  }
}

pub fn list_role_permissions(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "List role permissions".as_bytes().to_vec(),
  }
}

// Service-Roles
pub fn assign_role_to_service(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Assign role to service".as_bytes().to_vec(),
  }
}

pub fn remove_role_from_service(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Remove role from service".as_bytes().to_vec(),
  }
}

pub fn list_service_roles(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "List service roles".as_bytes().to_vec(),
  }
}

// Person-Service-Roles
pub fn assign_role_to_person_in_service(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Assign role to person in service".as_bytes().to_vec(),
  }
}

pub fn remove_role_from_person_in_service(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Remove role from person in service".as_bytes().to_vec(),
  }
}

pub fn list_person_roles_in_service(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "List person roles in service".as_bytes().to_vec(),
  }
}

pub fn list_persons_with_role_in_service(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "List persons with role in service".as_bytes().to_vec(),
  }
}

// Other checks
pub fn check_person_permission_in_service(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "Check person permission in service".as_bytes().to_vec(),
  }
}

pub fn list_services_of_person(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/plain".to_string(),
    content: "List services of person".as_bytes().to_vec(),
  }
}
