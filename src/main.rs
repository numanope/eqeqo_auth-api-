use httpageboy::{Rt, Server};
mod classes;
mod db;
mod handlers;

use crate::handlers::{
  create_role, create_user, delete_role, delete_user, get_user, list_roles, list_users, update_user,
};

fn main() {
  let serving_url: &str = "127.0.0.1:7878";
  // 127.0.0.1:7878
  let threads_number: u8 = 10;

  let mut server = Server::new(serving_url, threads_number, None).unwrap();
  server.add_route("/users", Rt::GET, list_users);
  server.add_route("/users", Rt::POST, create_user);
  server.add_route("/users/{id}", Rt::GET, get_user);
  server.add_route("/users/{id}", Rt::PUT, update_user);
  server.add_route("/users/{id}", Rt::DELETE, delete_user);
  server.add_route("/roles", Rt::GET, list_roles);
  server.add_route("/roles", Rt::POST, create_role);
  server.add_route("/roles", Rt::DELETE, delete_role);

  server.run();
}
