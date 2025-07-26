use crate::db::DB;
use httpageboy::{Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json; // Add for request body parsing
mod permissions;
mod relations;
mod roles;
mod services;
mod users;

// Helper to create a DB instance with test content
fn get_initialized_db() -> DB {
  let mut db = DB::new();
  db.test_content();
  db
}

pub fn home(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "text/html".to_string(),
    content: "<h1>Welcome to the Auth API</h1>".as_bytes().to_vec(),
  }
}

// Users
pub fn list_users(_req: &Request) -> Response {
  let db = get_initialized_db();
  let users = db.list_users();
  let json_response = json!(users).to_string();
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "application/json".to_string(),
    content: json_response.as_bytes().to_vec(),
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub fn create_user(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match serde_json::from_slice::<CreateUserPayload>(&req.body) {
    Ok(payload) => {
      // Generate a simple ID. In a real app, this would be more robust.
      let new_id = db.list_users().len() as i32 + 1;
      let new_user = User::new(
        new_id,
        payload.username,
        payload.password_hash,
        0, // Using 0 as a placeholder for created_at, since chrono is not explicitly allowed.
        None,
      );
      db.add_user(new_user.clone());
      Response {
        status: StatusCode::Created.to_string(),
        content_type: "application/json".to_string(),
        content: json!(new_user).to_string().as_bytes().to_vec(),
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn get_user(req: &Request) -> Response {
  let db = get_initialized_db();
  match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => match db.get_user(id) {
      Some(user) => Response {
        status: StatusCode::Ok.to_string(),
        content_type: "application/json".to_string(),
        content: json!(user).to_string().as_bytes().to_vec(),
      },
      None => Response {
        status: StatusCode::NotFound.to_string(),
        content_type: "text/plain".to_string(),
        content: "User not found".as_bytes().to_vec(),
      },
    },
    None => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: "Invalid user ID".as_bytes().to_vec(),
    },
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateUserPayload {
  username: Option<String>,
  password_hash: Option<String>,
  removed_at: Option<i64>,
}

pub fn update_user(req: &Request) -> Response {
  let mut db = get_initialized_db();
  let user_id = match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => id,
    None => {
      return Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: "text/plain".to_string(),
        content: "Invalid user ID".as_bytes().to_vec(),
      };
    }
  };

  match serde_json::from_slice::<UpdateUserPayload>(&req.body) {
    Ok(payload) => {
      let mut user_to_update = match db.get_user(user_id) {
        Some(user) => user,
        None => {
          return Response {
            status: StatusCode::NotFound.to_string(),
            content_type: "text/plain".to_string(),
            content: "User not found".as_bytes().to_vec(),
          };
        }
      };

      if let Some(username) = payload.username {
        user_to_update.username = username;
      }
      if let Some(password_hash) = payload.password_hash {
        user_to_update.set_password(password_hash);
      }
      if let Some(removed_at) = payload.removed_at {
        user_to_update.deactivate(removed_at);
      }

      match db.update_user(user_id, user_to_update.clone()) {
        Ok(_) => Response {
          status: StatusCode::Ok.to_string(),
          content_type: "application/json".to_string(),
          content: json!(user_to_update).to_string().as_bytes().to_vec(),
        },
        Err(e) => Response {
          status: StatusCode::InternalServerError.to_string(),
          content_type: "text/plain".to_string(),
          content: e.as_bytes().to_vec(),
        },
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn delete_user(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => match db.delete_user(id) {
      Ok(_) => Response {
        status: StatusCode::NoContent.to_string(),
        content_type: "text/plain".to_string(),
        content: "User deleted".as_bytes().to_vec(),
      },
      Err(e) => Response {
        status: StatusCode::NotFound.to_string(),
        content_type: "text/plain".to_string(),
        content: e.as_bytes().to_vec(),
      },
    },
    None => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: "Invalid user ID".as_bytes().to_vec(),
    },
  }
}

// Services
pub fn list_services(_req: &Request) -> Response {
  let db = get_initialized_db();
  let services = db.list_services();
  let json_response = json!(services).to_string();
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "application/json".to_string(),
    content: json_response.as_bytes().to_vec(),
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateServicePayload {
  name: String,
}

pub fn create_service(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match serde_json::from_slice::<CreateServicePayload>(&req.body) {
    Ok(payload) => {
      let new_id = db.list_services().len() as i32 + 1;
      let new_service = Service::new(new_id, payload.name);
      db.add_service(new_service.clone());
      Response {
        status: StatusCode::Created.to_string(),
        content_type: "application/json".to_string(),
        content: json!(new_service).to_string().as_bytes().to_vec(),
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateServicePayload {
  name: Option<String>,
}

pub fn update_service(req: &Request) -> Response {
  let mut db = get_initialized_db();
  let service_id = match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => id,
    None => {
      return Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: "text/plain".to_string(),
        content: "Invalid service ID".as_bytes().to_vec(),
      };
    }
  };

  match serde_json::from_slice::<UpdateServicePayload>(&req.body) {
    Ok(payload) => {
      let mut service_to_update = match db.get_service(service_id) {
        Some(service) => service,
        None => {
          return Response {
            status: StatusCode::NotFound.to_string(),
            content_type: "text/plain".to_string(),
            content: "Service not found".as_bytes().to_vec(),
          };
        }
      };

      if let Some(name) = payload.name {
        service_to_update.set_name(name);
      }

      match db.update_service(service_id, service_to_update.clone()) {
        Ok(_) => Response {
          status: StatusCode::Ok.to_string(),
          content_type: "application/json".to_string(),
          content: json!(service_to_update).to_string().as_bytes().to_vec(),
        },
        Err(e) => Response {
          status: StatusCode::InternalServerError.to_string(),
          content_type: "text/plain".to_string(),
          content: e.as_bytes().to_vec(),
        },
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn delete_service(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => match db.delete_service(id) {
      Ok(_) => Response {
        status: StatusCode::NoContent.to_string(),
        content_type: "text/plain".to_string(),
        content: "Service deleted".as_bytes().to_vec(),
      },
      Err(e) => Response {
        status: StatusCode::NotFound.to_string(),
        content_type: "text/plain".to_string(),
        content: e.as_bytes().to_vec(),
      },
    },
    None => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: "Invalid service ID".as_bytes().to_vec(),
    },
  }
}

// Roles
pub fn list_roles(_req: &Request) -> Response {
  let db = get_initialized_db();
  let roles = db.list_roles();
  let json_response = json!(roles).to_string();
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "application/json".to_string(),
    content: json_response.as_bytes().to_vec(),
  }
}

pub fn get_role(req: &Request) -> Response {
  let db = get_initialized_db();
  match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => match db.get_role(id) {
      Some(role) => Response {
        status: StatusCode::Ok.to_string(),
        content_type: "application/json".to_string(),
        content: json!(role).to_string().as_bytes().to_vec(),
      },
      None => Response {
        status: StatusCode::NotFound.to_string(),
        content_type: "text/plain".to_string(),
        content: "Role not found".as_bytes().to_vec(),
      },
    },
    None => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: "Invalid role ID".as_bytes().to_vec(),
    },
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateRolePayload {
  name: String,
}

pub fn create_role(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match serde_json::from_slice::<CreateRolePayload>(&req.body) {
    Ok(payload) => {
      let new_id = db.list_roles().len() as i32 + 1;
      let new_role = Role::new(new_id, payload.name);
      db.add_role(new_role.clone());
      Response {
        status: StatusCode::Created.to_string(),
        content_type: "application/json".to_string(),
        content: json!(new_role).to_string().as_bytes().to_vec(),
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateRolePayload {
  name: Option<String>,
}

pub fn update_role(req: &Request) -> Response {
  let mut db = get_initialized_db();
  let role_id = match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => id,
    None => {
      return Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: "text/plain".to_string(),
        content: "Invalid role ID".as_bytes().to_vec(),
      };
    }
  };

  match serde_json::from_slice::<UpdateRolePayload>(&req.body) {
    Ok(payload) => {
      let mut role_to_update = match db.get_role(role_id) {
        Some(role) => role,
        None => {
          return Response {
            status: StatusCode::NotFound.to_string(),
            content_type: "text/plain".to_string(),
            content: "Role not found".as_bytes().to_vec(),
          };
        }
      };

      if let Some(name) = payload.name {
        role_to_update.set_name(name);
      }

      match db.update_role(role_id, role_to_update.clone()) {
        Ok(_) => Response {
          status: StatusCode::Ok.to_string(),
          content_type: "application/json".to_string(),
          content: json!(role_to_update).to_string().as_bytes().to_vec(),
        },
        Err(e) => Response {
          status: StatusCode::InternalServerError.to_string(),
          content_type: "text/plain".to_string(),
          content: e.as_bytes().to_vec(),
        },
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn delete_role(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => match db.delete_role(id) {
      Ok(_) => Response {
        status: StatusCode::NoContent.to_string(),
        content_type: "text/plain".to_string(),
        content: "Role deleted".as_bytes().to_vec(),
      },
      Err(e) => Response {
        status: StatusCode::NotFound.to_string(),
        content_type: "text/plain".to_string(),
        content: e.as_bytes().to_vec(),
      },
    },
    None => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: "Invalid role ID".as_bytes().to_vec(),
    },
  }
}

// Permissions
pub fn list_permissions(_req: &Request) -> Response {
  let db = get_initialized_db();
  let permissions = db.list_permissions();
  let json_response = json!(permissions).to_string();
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "application/json".to_string(),
    content: json_response.as_bytes().to_vec(),
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreatePermissionPayload {
  name: String,
}

pub fn create_permission(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match serde_json::from_slice::<CreatePermissionPayload>(&req.body) {
    Ok(payload) => {
      let new_id = db.list_permissions().len() as i32 + 1;
      let new_permission = Permission::new(new_id, payload.name);
      db.add_permission(new_permission.clone());
      Response {
        status: StatusCode::Created.to_string(),
        content_type: "application/json".to_string(),
        content: json!(new_permission).to_string().as_bytes().to_vec(),
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdatePermissionPayload {
  name: Option<String>,
}

pub fn update_permission(req: &Request) -> Response {
  let mut db = get_initialized_db();
  let permission_id = match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => id,
    None => {
      return Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: "text/plain".to_string(),
        content: "Invalid permission ID".as_bytes().to_vec(),
      };
    }
  };

  match serde_json::from_slice::<UpdatePermissionPayload>(&req.body) {
    Ok(payload) => {
      let mut permission_to_update = match db.get_permission(permission_id) {
        Some(permission) => permission,
        None => {
          return Response {
            status: StatusCode::NotFound.to_string(),
            content_type: "text/plain".to_string(),
            content: "Permission not found".as_bytes().to_vec(),
          };
        }
      };

      if let Some(name) = payload.name {
        permission_to_update.set_name(name);
      }

      match db.update_permission(permission_id, permission_to_update.clone()) {
        Ok(_) => Response {
          status: StatusCode::Ok.to_string(),
          content_type: "application/json".to_string(),
          content: json!(permission_to_update).to_string().as_bytes().to_vec(),
        },
        Err(e) => Response {
          status: StatusCode::InternalServerError.to_string(),
          content_type: "text/plain".to_string(),
          content: e.as_bytes().to_vec(),
        },
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn delete_permission(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => match db.delete_permission(id) {
      Ok(_) => Response {
        status: StatusCode::NoContent.to_string(),
        content_type: "text/plain".to_string(),
        content: "Permission deleted".as_bytes().to_vec(),
      },
      Err(e) => Response {
        status: StatusCode::NotFound.to_string(),
        content_type: "text/plain".to_string(),
        content: e.as_bytes().to_vec(),
      },
    },
    None => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: "Invalid permission ID".as_bytes().to_vec(),
    },
  }
}

// Role-Permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RolePermissionPayload {
  role_id: i32,
  permission_id: i32,
}

pub fn assign_permission_to_role(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match serde_json::from_slice::<RolePermissionPayload>(&req.body) {
    Ok(payload) => match db.assign_permission_to_role(payload.role_id, payload.permission_id) {
      Ok(_) => Response {
        status: StatusCode::Ok.to_string(),
        content_type: "text/plain".to_string(),
        content: "Permission assigned to role".as_bytes().to_vec(),
      },
      Err(e) => Response {
        status: StatusCode::Conflict.to_string(),
        content_type: "text/plain".to_string(),
        content: e.as_bytes().to_vec(),
      },
    },
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn remove_permission_from_role(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match serde_json::from_slice::<RolePermissionPayload>(&req.body) {
    Ok(payload) => match db.remove_permission_from_role(payload.role_id, payload.permission_id) {
      Ok(_) => Response {
        status: StatusCode::Ok.to_string(),
        content_type: "text/plain".to_string(),
        content: "Permission removed from role".as_bytes().to_vec(),
      },
      Err(e) => Response {
        status: StatusCode::NotFound.to_string(),
        content_type: "text/plain".to_string(),
        content: e.as_bytes().to_vec(),
      },
    },
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn list_role_permissions(req: &Request) -> Response {
  let db = get_initialized_db();
  match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(role_id) => {
      let permissions = db.list_permissions_for_role(role_id);
      Response {
        status: StatusCode::Ok.to_string(),
        content_type: "application/json".to_string(),
        content: json!(permissions).to_string().as_bytes().to_vec(),
      }
    }
    None => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: "Invalid role ID".as_bytes().to_vec(),
    },
  }
}

// Service-Roles
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ServiceRolePayload {
  service_id: i32,
  role_id: i32,
}

pub fn assign_role_to_service(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match serde_json::from_slice::<ServiceRolePayload>(&req.body) {
    Ok(payload) => match db.assign_role_to_service(payload.service_id, payload.role_id) {
      Ok(_) => Response {
        status: StatusCode::Ok.to_string(),
        content_type: "text/plain".to_string(),
        content: "Role assigned to service".as_bytes().to_vec(),
      },
      Err(e) => Response {
        status: StatusCode::Conflict.to_string(),
        content_type: "text/plain".to_string(),
        content: e.as_bytes().to_vec(),
      },
    },
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn remove_role_from_service(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match serde_json::from_slice::<ServiceRolePayload>(&req.body) {
    Ok(payload) => match db.remove_role_from_service(payload.service_id, payload.role_id) {
      Ok(_) => Response {
        status: StatusCode::Ok.to_string(),
        content_type: "text/plain".to_string(),
        content: "Role removed from service".as_bytes().to_vec(),
      },
      Err(e) => Response {
        status: StatusCode::NotFound.to_string(),
        content_type: "text/plain".to_string(),
        content: e.as_bytes().to_vec(),
      },
    },
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn list_service_roles(req: &Request) -> Response {
  let db = get_initialized_db();
  match req
    .params
    .get("id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(service_id) => {
      let roles = db.list_roles_for_service(service_id);
      Response {
        status: StatusCode::Ok.to_string(),
        content_type: "application/json".to_string(),
        content: json!(roles).to_string().as_bytes().to_vec(),
      }
    }
    None => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: "Invalid service ID".as_bytes().to_vec(),
    },
  }
}

// Person-Service-Roles
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersonServiceRolePayload {
  person_id: i32,
  service_id: i32,
  role_id: i32,
}

pub fn assign_role_to_person_in_service(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match serde_json::from_slice::<PersonServiceRolePayload>(&req.body) {
    Ok(payload) => {
      match db.assign_role_to_person_in_service(
        payload.person_id,
        payload.service_id,
        payload.role_id,
      ) {
        Ok(_) => Response {
          status: StatusCode::Ok.to_string(),
          content_type: "text/plain".to_string(),
          content: "Role assigned to person in service".as_bytes().to_vec(),
        },
        Err(e) => Response {
          status: StatusCode::Conflict.to_string(),
          content_type: "text/plain".to_string(),
          content: e.as_bytes().to_vec(),
        },
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn remove_role_from_person_in_service(req: &Request) -> Response {
  let mut db = get_initialized_db();
  match serde_json::from_slice::<PersonServiceRolePayload>(&req.body) {
    Ok(payload) => {
      match db.remove_role_from_person_in_service(
        payload.person_id,
        payload.service_id,
        payload.role_id,
      ) {
        Ok(_) => Response {
          status: StatusCode::Ok.to_string(),
          content_type: "text/plain".to_string(),
          content: "Role removed from person in service".as_bytes().to_vec(),
        },
        Err(e) => Response {
          status: StatusCode::NotFound.to_string(),
          content_type: "text/plain".to_string(),
          content: e.as_bytes().to_vec(),
        },
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn list_person_roles_in_service(req: &Request) -> Response {
  let db = get_initialized_db();
  let person_id = match req
    .params
    .get("person_id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => id,
    None => {
      return Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: "text/plain".to_string(),
        content: "Invalid person ID".as_bytes().to_vec(),
      };
    }
  };
  let service_id = match req
    .params
    .get("service_id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => id,
    None => {
      return Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: "text/plain".to_string(),
        content: "Invalid service ID".as_bytes().to_vec(),
      };
    }
  };

  let roles = db.list_person_roles_in_service(person_id, service_id);
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "application/json".to_string(),
    content: json!(roles).to_string().as_bytes().to_vec(),
  }
}

pub fn list_persons_with_role_in_service(req: &Request) -> Response {
  let db = get_initialized_db();
  let service_id = match req
    .params
    .get("service_id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => id,
    None => {
      return Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: "text/plain".to_string(),
        content: "Invalid service ID".as_bytes().to_vec(),
      };
    }
  };
  let role_id = match req
    .params
    .get("role_id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(id) => id,
    None => {
      return Response {
        status: StatusCode::BadRequest.to_string(),
        content_type: "text/plain".to_string(),
        content: "Invalid role ID".as_bytes().to_vec(),
      };
    }
  };

  let persons = db.list_persons_with_role_in_service(service_id, role_id);
  Response {
    status: StatusCode::Ok.to_string(),
    content_type: "application/json".to_string(),
    content: json!(persons).to_string().as_bytes().to_vec(),
  }
}

// Other checks
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CheckPermissionPayload {
  person_id: i32,
  service_id: i32,
  permission_id: i32,
}

pub fn check_person_permission_in_service(req: &Request) -> Response {
  let db = get_initialized_db();
  match serde_json::from_slice::<CheckPermissionPayload>(&req.body) {
    Ok(payload) => {
      let has_permission = db.check_person_permission_in_service(
        payload.person_id,
        payload.service_id,
        payload.permission_id,
      );
      Response {
        status: StatusCode::Ok.to_string(),
        content_type: "application/json".to_string(),
        content: json!({"has_permission": has_permission})
          .to_string()
          .as_bytes()
          .to_vec(),
      }
    }
    Err(e) => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: format!("Failed to parse request body: {}", e)
        .as_bytes()
        .to_vec(),
    },
  }
}

pub fn list_services_of_person(req: &Request) -> Response {
  let db = get_initialized_db();
  match req
    .params
    .get("person_id")
    .and_then(|id_str| id_str.parse::<i32>().ok())
  {
    Some(person_id) => {
      let services = db.list_services_of_person(person_id);
      Response {
        status: StatusCode::Ok.to_string(),
        content_type: "application/json".to_string(),
        content: json!(services).to_string().as_bytes().to_vec(),
      }
    }
    None => Response {
      status: StatusCode::BadRequest.to_string(),
      content_type: "text/plain".to_string(),
      content: "Invalid person ID".as_bytes().to_vec(),
    },
  }
}
