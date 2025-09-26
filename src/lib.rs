use httpageboy::{handler, Request, Response, Rt, Server, StatusCode};

mod handlers;
use crate::handlers::*;

pub async fn auth_server(url: &str, _threads_number: u8) -> Server {
    let mut server = Server::new(url, None)
        .await
        .expect("Failed to create server");

    server.add_route("/", Rt::GET, handler!(home));

    // Users
    server.add_route("/users", Rt::GET, handler!(list_users));
    server.add_route("/users", Rt::POST, handler!(create_user));
    server.add_route("/users/:id", Rt::GET, handler!(get_user));
    server.add_route("/users/:id", Rt::PUT, handler!(update_user));
    server.add_route("/users/:id", Rt::DELETE, handler!(delete_user));

    // Services
    server.add_route("/services", Rt::GET, handler!(list_services));
    server.add_route("/services", Rt::POST, handler!(create_service));
    server.add_route("/services/:id", Rt::GET, handler!(get_service));
    server.add_route("/services/:id", Rt::PUT, handler!(update_service));
    server.add_route("/services/:id", Rt::DELETE, handler!(delete_service));

    // Roles
    server.add_route("/roles", Rt::GET, handler!(list_roles));
    server.add_route("/roles", Rt::POST, handler!(create_role));
    server.add_route("/roles/:id", Rt::GET, handler!(get_role));
    server.add_route("/roles/:id", Rt::PUT, handler!(update_role));
    server.add_route("/roles/:id", Rt::DELETE, handler!(delete_role));

    // Permissions
    server.add_route("/permissions", Rt::GET, handler!(list_permissions));
    server.add_route("/permissions", Rt::POST, handler!(create_permission));
    server.add_route("/permissions/:id", Rt::GET, handler!(get_permission));
    server.add_route("/permissions/:id", Rt::PUT, handler!(update_permission));
    server.add_route("/permissions/:id", Rt::DELETE, handler!(delete_permission));

    // Role-Permissions
    server.add_route("/role-permissions", Rt::POST, handler!(assign_permission_to_role));
    server.add_route("/role-permissions", Rt::DELETE, handler!(remove_permission_from_role));
    server.add_route("/roles/:id/permissions", Rt::GET, handler!(list_role_permissions));

    // Service-Roles
    server.add_route("/service-roles", Rt::POST, handler!(assign_role_to_service));
    server.add_route("/service-roles", Rt::DELETE, handler!(remove_role_from_service));
    server.add_route("/services/:id/roles", Rt::GET, handler!(list_service_roles));

    // Person-Service-Roles
    server.add_route("/person-service-roles", Rt::POST, handler!(assign_role_to_person_in_service));
    server.add_route("/person-service-roles", Rt::DELETE, handler!(remove_role_from_person_in_service));
    server.add_route("/people/:person_id/services/:service_id/roles", Rt::GET, handler!(list_person_roles_in_service));
    server.add_route("/services/:service_id/roles/:role_id/people", Rt::GET, handler!(list_persons_with_role_in_service));

    // Other checks
    server.add_route("/check-permission", Rt::GET, handler!(check_person_permission_in_service));
    server.add_route("/people/:person_id/services", Rt::GET, handler!(list_services_of_person));

    server
}