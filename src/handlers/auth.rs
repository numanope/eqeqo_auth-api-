use hyper::{Body, Request, Response};
use crate::services::auth::{register_user, login_user, get_user_from_token};

pub async fn register(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  let body = hyper::body::to_bytes(req.into_body()).await?;
  let (username, password) = parse_credentials(&String::from_utf8_lossy(&body));
  match register_user(username, password) {
    Ok(_) => Ok(Response::new(Body::from("User registered"))),
    Err(e) => Ok(Response::builder().status(400).body(Body::from(e)).unwrap())
  }
}

pub async fn login(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  let body = hyper::body::to_bytes(req.into_body()).await?;
  let (username, password) = parse_credentials(&String::from_utf8_lossy(&body));
  match login_user(username, password) {
    Some(token) => Ok(Response::new(Body::from(token))),
    None => Ok(Response::builder().status(401).body(Body::from("Invalid credentials")).unwrap())
  }
}

pub async fn me(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
  let token = req.headers()
    .get("Authorization")
    .and_then(|v| v.to_str().ok())
    .unwrap_or("");
  match get_user_from_token(token) {
    Some(user) => Ok(Response::new(Body::from(user))),
    None => Ok(Response::builder().status(401).body(Body::from("Invalid token")).unwrap())
  }
}

fn parse_credentials(body: &str) -> (String, String) {
  let mut username = String::new();
  let mut password = String::new();

  for part in body.split('&') {
    if let Some(val) = part.strip_prefix("username=") {
      username = val.to_string();
    } 

    else if let Some(val) = part.strip_prefix("password=") {
      password = val.to_string();
    }
  }

  (username, password)
}
