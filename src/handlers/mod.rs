use httpageboy::{Request, Response, StatusCode};

// Module declarations
pub mod permissions;
pub mod relations;
pub mod roles;
pub mod services;
pub mod users;

// Publicly export all handlers
pub use permissions::*;
pub use relations::*;
pub use roles::*;
pub use services::*;
pub use users::*;

// General handler
pub async fn home(_req: &Request) -> Response {
  Response {
    status: StatusCode::Ok.to_string(),
    content: "Welcome to the Auth API".as_bytes().to_vec(),
    content_type: "text/plain; charset=utf-8".to_string(),
  }
}