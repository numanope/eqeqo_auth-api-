use httpageboy::{Rt, Server};
mod classes;
mod db;
mod handlers;
use crate::handlers::*;

pub fn auth_server(url: &str, _threads_number: u8) -> Server {
  let mut server = Server::new(url, _threads_number, None).expect("Failed to create server");

  server.add_route("/", Rt::GET, home);

  // Users
  server.add_route("/users", Rt::GET, list_users);
  server.add_route("/users", Rt::POST, create_user); // Needs body // Needs body
  server.add_route("/users/{id}", Rt::GET, get_user);
  server.add_route("/users/{id}", Rt::PUT, update_user); // Needs body
  server.add_route("/users/{id}", Rt::DELETE, delete_user);

  // Services
  server.add_route("/services", Rt::GET, list_services);
  server.add_route("/services", Rt::POST, create_service); // Needs body
  server.add_route("/services/{id}", Rt::PUT, update_service); // Needs body
  server.add_route("/services/{id}", Rt::DELETE, delete_service);

  // Roles
  server.add_route("/roles", Rt::GET, list_roles);
  server.add_route("/roles", Rt::POST, create_role); // Needs body
  server.add_route("/roles/{id}", Rt::GET, get_role);
  server.add_route("/roles/{id}", Rt::PUT, update_role); // Needs body
  server.add_route("/roles/{id}", Rt::DELETE, delete_role);

  // Permissions
  server.add_route("/permissions", Rt::GET, list_permissions);
  server.add_route("/permissions", Rt::POST, create_permission); // Needs body
  server.add_route("/permissions/{id}", Rt::PUT, update_permission); // Needs body
  server.add_route("/permissions/{id}", Rt::DELETE, delete_permission);

  // Role-Permissions
  server.add_route("/role-permissions", Rt::POST, assign_permission_to_role); // Needs body
  server.add_route("/role-permissions", Rt::DELETE, remove_permission_from_role);
  server.add_route("/roles/{id}/permissions", Rt::GET, list_role_permissions);

  // Service-Roles
  server.add_route("/service-roles", Rt::POST, assign_role_to_service); // Needs body
  server.add_route("/service-roles", Rt::DELETE, remove_role_from_service);
  server.add_route("/services/{id}/roles", Rt::GET, list_service_roles);

  // Person-Service-Roles
  server.add_route(
    "/person-service-roles",
    Rt::POST, // Needs body
    assign_role_to_person_in_service,
  );
  server.add_route(
    "/person-service-roles",
    Rt::DELETE,
    remove_role_from_person_in_service,
  );
  server.add_route(
    "/people/{person_id}/services/{service_id}/roles",
    Rt::GET,
    list_person_roles_in_service,
  );
  server.add_route(
    "/services/{service_id}/roles/{role_id}/people",
    Rt::GET,
    list_persons_with_role_in_service,
  );

  // Other checks
  server.add_route(
    "/check-permission",
    Rt::GET,
    check_person_permission_in_service,
  );
  server.add_route(
    "/people/{person_id}/services",
    Rt::GET,
    list_services_of_person,
  );

  server
}
