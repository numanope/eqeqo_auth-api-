#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::sync::Once;
    use std::thread;
    use std::time::Duration;

    use httpageboy::{Request, Response, Rh, Rt, Server, StatusCode};

    // ---- Constantes ----
    const SERVER_URL: &str = "127.0.0.1:7878";
    const POOL_SIZE: u8 = 10;
    const INTERVAL: Duration = Duration::from_millis(250);

    static INIT: Once = Once::new();

    // ---- Handlers de ejemplo (los mismos que en main.rs de auth-api) ----

    fn list_users(_req: &Request) -> Response {
        Response {
            status: StatusCode::Ok.to_string(),
            content_type: String::new(),
            content: "Demo: Listar usuarios".as_bytes().to_vec(),
        }
    }

    fn get_user(_req: &Request) -> Response {
        Response {
            status: StatusCode::Ok.to_string(),
            content_type: String::new(),
            content: "Demo: Ver perfil de usuario".as_bytes().to_vec(),
        }
    }

    fn create_user(_req: &Request) -> Response {
        Response {
            status: StatusCode::Ok.to_string(),
            content_type: String::new(),
            content: "Demo: Crear usuario".as_bytes().to_vec(),
        }
    }

    fn update_user(_req: &Request) -> Response {
        Response {
            status: StatusCode::Ok.to_string(),
            content_type: String::new(),
            content: "Demo: Editar usuario".as_bytes().to_vec(),
        }
    }

    fn delete_user(_req: &Request) -> Response {
        Response {
            status: StatusCode::Ok.to_string(),
            content_type: String::new(),
            content: "Demo: Eliminar usuario".as_bytes().to_vec(),
        }
    }

    fn list_roles(_req: &Request) -> Response {
        Response {
            status: StatusCode::Ok.to_string(),
            content_type: String::new(),
            content: "Demo: Listar roles".as_bytes().to_vec(),
        }
    }

    fn create_role(_req: &Request) -> Response {
        Response {
            status: StatusCode::Ok.to_string(),
            content_type: String::new(),
            content: "Demo: Crear rol".as_bytes().to_vec(),
        }
    }

    // ---- Setup del servidor (se ejecuta una sola vez) ----

    fn setup() {
        INIT.call_once(|| {
            thread::spawn(|| {
                // Construir el HashMap<(Rt, String), Rh> con todas las rutas
                let mut routes: HashMap<(Rt, String), Rh> = HashMap::new();
                routes.insert(
                    (Rt::GET, "/users".to_string()),
                    Rh {
                        handler: list_users,
                    },
                );
                routes.insert(
                    (Rt::GET, "/users/{id}".to_string()),
                    Rh { handler: get_user },
                );
                routes.insert(
                    (Rt::POST, "/users".to_string()),
                    Rh {
                        handler: create_user,
                    },
                );
                routes.insert(
                    (Rt::PUT, "/users/{id}".to_string()),
                    Rh {
                        handler: update_user,
                    },
                );
                routes.insert(
                    (Rt::DELETE, "/users/{id}".to_string()),
                    Rh {
                        handler: delete_user,
                    },
                );
                routes.insert(
                    (Rt::GET, "/roles".to_string()),
                    Rh {
                        handler: list_roles,
                    },
                );
                routes.insert(
                    (Rt::POST, "/roles".to_string()),
                    Rh {
                        handler: create_role,
                    },
                );

                // Arrancar el servidor con las rutas definidas
                let server = Server::new(SERVER_URL, POOL_SIZE, Some(routes))
                    .expect("No se pudo crear el servidor de pruebas");
                server.run();
            });
            // Esperar un instante para que el servidor esté listo
            thread::sleep(INTERVAL);
        });
    }

    // Envía la petición y compara que la respuesta contenga el fragmento esperado
    fn test_server(request: &[u8], expected_fragment: &[u8]) {
        // Conectar al servidor
        let mut stream =
            TcpStream::connect(SERVER_URL).expect("No se pudo conectar al servidor de pruebas");
        // Enviar la petición cruda
        stream.write_all(request).unwrap();
        // Leer toda la respuesta
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();

        let response_str = String::from_utf8_lossy(&buffer);
        let expected_str = String::from_utf8_lossy(expected_fragment);

        assert!(
            response_str.contains(&expected_str.as_ref()),
            "RESPUESTA:\n{}\n\nNO contiene:\n{}\n",
            response_str,
            expected_str
        );
    }

    fn run_test(request: &[u8], expected_fragment: &[u8]) {
        setup();
        test_server(request, expected_fragment);
    }

    // ---- Tests de cada ruta ----

    #[test]
    fn test_list_users() {
        let request = b"GET /users HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let expected_fragment = b"Demo: Listar usuarios";
        run_test(request, expected_fragment);
    }

    #[test]
    fn test_get_user() {
        // Aquí el path es literal "/users/{id}", sin reemplazar {id}
        let request = b"GET /users/{id} HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let expected_fragment = b"Demo: Ver perfil de usuario";
        run_test(request, expected_fragment);
    }

    #[test]
    fn test_create_user() {
        let request = b"POST /users HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let expected_fragment = b"Demo: Crear usuario";
        run_test(request, expected_fragment);
    }

    #[test]
    fn test_update_user() {
        let request = b"PUT /users/{id} HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let expected_fragment = b"Demo: Editar usuario";
        run_test(request, expected_fragment);
    }

    #[test]
    fn test_delete_user() {
        let request = b"DELETE /users/{id} HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let expected_fragment = b"Demo: Eliminar usuario";
        run_test(request, expected_fragment);
    }

    #[test]
    fn test_list_roles() {
        let request = b"GET /roles HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let expected_fragment = b"Demo: Listar roles";
        run_test(request, expected_fragment);
    }

    #[test]
    fn test_create_role() {
        let request = b"POST /roles HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let expected_fragment = b"Demo: Crear rol";
        run_test(request, expected_fragment);
    }
}
