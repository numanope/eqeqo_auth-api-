mod tests {
  use httpageboy::Server;
  use httpageboy::test_utils::{POOL_SIZE, SERVER_URL, run_test, setup_test_server};

  fn create_test_server() -> Server {
    let server = Server::new(SERVER_URL, POOL_SIZE, None).unwrap();
    server
  }

  #[test]
  fn test_home() {
    setup_test_server(|| create_test_server());
    let req = b"GET / HTTP/1.1\r\n\r\n";
    run_test(req, b"Welcome to the Auth API");
  }

  // Users
  #[test]
  fn test_list_users_success() {
    setup_test_server(|| create_test_server());
    let req = b"GET /users/ HTTP/1.1\r\n\r\n";
    run_test(req, b"List users");
  }

  #[test]
  fn test_get_user_not_found() {
    setup_test_server(|| create_test_server());
    let req = b"GET /users/999 HTTP/1.1\r\n\r\n";
    run_test(req, b"404");
  }

  #[test]
  fn test_create_user_success() {
    setup_test_server(|| create_test_server());
    let req = b"POST /users/ HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"username\":\"a\",\"password\":\"b\"}";
    run_test(req, b"Create user");
  }

  #[test]
  fn test_update_user_invalid_id() {
    setup_test_server(|| create_test_server());
    let req =
      b"PUT /users/abc HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"username\":\"x\"}";
    run_test(req, b"404");
  }

  #[test]
  fn test_delete_user_success() {
    setup_test_server(|| create_test_server());
    let req = b"DELETE /users/1 HTTP/1.1\r\n\r\n";
    run_test(req, b"Delete user");
  }

  // Services
  #[test]
  fn test_list_services() {
    setup_test_server(|| create_test_server());
    let req = b"GET /services/ HTTP/1.1\r\n\r\n";
    run_test(req, b"List services");
  }

  #[test]
  fn test_create_service_missing_body() {
    setup_test_server(|| create_test_server());
    let req = b"POST /services/ HTTP/1.1\r\n\r\n";
    run_test(req, b"400");
  }

  #[test]
  fn test_update_service() {
    setup_test_server(|| create_test_server());
    let req =
      b"PUT /services/1 HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"name\":\"svc\"}";
    run_test(req, b"Update service");
  }

  #[test]
  fn test_delete_service_not_found() {
    setup_test_server(|| create_test_server());
    let req = b"DELETE /services/999 HTTP/1.1\r\n\r\n";
    run_test(req, b"404");
  }

  // Roles
  #[test]
  fn test_list_roles() {
    setup_test_server(|| create_test_server());
    let req = b"GET /roles/ HTTP/1.1\r\n\r\n";
    run_test(req, b"List roles");
  }

  #[test]
  fn test_get_role_success() {
    setup_test_server(|| create_test_server());
    let req = b"GET /roles/1 HTTP/1.1\r\n\r\n";
    run_test(req, b"Get role");
  }

  #[test]
  fn test_create_role_conflict() {
    setup_test_server(|| create_test_server());
    let req =
      b"POST /roles/ HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"name\":\"existing\"}";
    run_test(req, b"409");
  }

  #[test]
  fn test_update_role() {
    setup_test_server(|| create_test_server());
    let req = b"PUT /roles/1 HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"name\":\"new\"}";
    run_test(req, b"Update role");
  }

  #[test]
  fn test_delete_role() {
    setup_test_server(|| create_test_server());
    let req = b"DELETE /roles/1 HTTP/1.1\r\n\r\n";
    run_test(req, b"Delete role");
  }

  // Permissions
  #[test]
  fn test_list_permissions() {
    setup_test_server(|| create_test_server());
    let req = b"GET /permissions/ HTTP/1.1\r\n\r\n";
    run_test(req, b"List permissions");
  }

  #[test]
  fn test_create_permission() {
    setup_test_server(|| create_test_server());
    let req =
      b"POST /permissions/ HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"name\":\"p\"}";
    run_test(req, b"Create permission");
  }

  #[test]
  fn test_update_permission() {
    setup_test_server(|| create_test_server());
    let req =
      b"PUT /permissions/1 HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"name\":\"p2\"}";
    run_test(req, b"Update permission");
  }

  #[test]
  fn test_delete_permission() {
    setup_test_server(|| create_test_server());
    let req = b"DELETE /permissions/1 HTTP/1.1\r\n\r\n";
    run_test(req, b"Delete permission");
  }

  // Role-Permissions
  #[test]
  fn test_assign_permission_to_role() {
    setup_test_server(|| create_test_server());
    let req = b"POST /role-permissions HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"role_id\":1,\"permission_id\":2}";
    run_test(req, b"Assign permission to role");
  }

  #[test]
  fn test_remove_permission_from_role() {
    setup_test_server(|| create_test_server());
    let req = b"DELETE /role-permissions HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"role_id\":1,\"permission_id\":2}";
    run_test(req, b"Remove permission from role");
  }

  #[test]
  fn test_list_role_permissions() {
    setup_test_server(|| create_test_server());
    let req = b"GET /roles/1/permissions HTTP/1.1\r\n\r\n";
    run_test(req, b"List role permissions");
  }

  // Service-Roles
  #[test]
  fn test_assign_role_to_service() {
    setup_test_server(|| create_test_server());
    let req = b"POST /service-roles HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"service_id\":1,\"role_id\":2}";
    run_test(req, b"Assign role to service");
  }

  #[test]
  fn test_remove_role_from_service() {
    setup_test_server(|| create_test_server());
    let req = b"DELETE /service-roles HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"service_id\":1,\"role_id\":2}";
    run_test(req, b"Remove role from service");
  }

  #[test]
  fn test_list_service_roles() {
    setup_test_server(|| create_test_server());
    let req = b"GET /services/1/roles HTTP/1.1\r\n\r\n";
    run_test(req, b"List service roles");
  }

  // Person-Service-Roles
  #[test]
  fn test_assign_role_to_person_in_service() {
    setup_test_server(|| create_test_server());
    let req = b"POST /person-service-roles HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"person_id\":1,\"service_id\":2,\"role_id\":3}";
    run_test(req, b"Assign role to person in service");
  }

  #[test]
  fn test_remove_role_from_person_in_service() {
    setup_test_server(|| create_test_server());
    let req = b"DELETE /person-service-roles HTTP/1.1\r\nContent-Type: application/json\r\n\r\n{\"person_id\":1,\"service_id\":2,\"role_id\":3}";
    run_test(req, b"Remove role from person in service");
  }

  #[test]
  fn test_list_person_roles_in_service() {
    setup_test_server(|| create_test_server());
    let req = b"GET /people/1/services/2/roles HTTP/1.1\r\n\r\n";
    run_test(req, b"List person roles in service");
  }

  #[test]
  fn test_list_persons_with_role_in_service() {
    setup_test_server(|| create_test_server());
    let req = b"GET /services/2/roles/3/people HTTP/1.1\r\n\r\n";
    run_test(req, b"List persons with role in service");
  }

  // Checks
  #[test]
  fn test_check_person_permission_in_service() {
    setup_test_server(|| create_test_server());
    let req = b"GET /check-permission?person_id=1&service_id=2&permission=read HTTP/1.1\r\n\r\n";
    run_test(req, b"Check person permission in service");
  }

  #[test]
  fn test_list_services_of_person() {
    setup_test_server(|| create_test_server());
    let req = b"GET /people/1/services HTTP/1.1\r\n\r\n";
    run_test(req, b"List services of person");
  }
}
