use auth_api::auth_server;

fn main() {
  let server = auth_server("127.0.0.1:7878", 10);
  server.run();
}
