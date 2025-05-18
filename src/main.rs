use hyper::{Request, Response};
use hyper::body::Incoming as Body;
use hyper::service::service_fn;
use hyper::server::conn::http1;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;

mod handlers;
mod models;
mod services;
mod utils;

async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  let path = req.uri().path();
  let method = req.method();

  match (method.as_str(), path) {
    ("POST", "/register") => handlers::auth::register(req).await,
    ("POST", "/login") => handlers::auth::login(req).await,
    ("GET", "/me") => handlers::auth::me(req).await,
    _ => Ok(Response::builder()
      .status(404)
      .body(Body::from("Not Found"))
      .unwrap())
  }
}

async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let listener = TcpListener::bind("127.0.0.1:3000").await?;

  loop {
    let (stream, _) = listener.accept().await?;

    tokio::task::spawn(async move {
      let svc = service_fn(handle_request);
      if let Err(err) = http1::Builder::new()
        .serve_connection(stream, svc)
        .await
      {
        eprintln!("server error: {}", err);
      }
    });
  }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  router(req).await
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let rt = Runtime::new()?;
  rt.block_on(run())
}
