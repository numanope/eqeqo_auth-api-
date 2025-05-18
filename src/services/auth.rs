use std::sync::Mutex;
use crate::models::user::User;
use crate::utils::hash::{hash_password, verify_password};
use std::collections::HashMap;

static mut USERS: Option<Mutex<HashMap<String, User>>> = None;
static mut TOKENS: Option<Mutex<HashMap<String, String>>> = None;

fn init() {
	unsafe {
		if USERS.is_none() {
			USERS = Some(Mutex::new(HashMap::new()));
			TOKENS = Some(Mutex::new(HashMap::new()));
		}
	}
}

pub fn register_user(username: String, password: String) -> Result<(), &'static str> {
	init();
	let mut users = unsafe { USERS.as_ref().unwrap().lock().unwrap() };
	if users.contains_key(&username) {
		return Err("User exists");
	}
	let hash = hash_password(&password);
	let user = User { username: username.clone(), password_hash: hash };
	users.insert(username, user);
	Ok(())
}

pub fn login_user(username: String, password: String) -> Option<String> {
	init();
	let users = unsafe { USERS.as_ref().unwrap().lock().unwrap() };
	let user = users.get(&username)?;
	if !verify_password(&password, &user.password_hash) {
		return None;
	}
	let token = generate_token();
	let mut tokens = unsafe { TOKENS.as_ref().unwrap().lock().unwrap() };
	tokens.insert(token.clone(), username);
	Some(token)
}

pub fn get_user_from_token(token: &str) -> Option<String> {
	init();
	let tokens = unsafe { TOKENS.as_ref().unwrap().lock().unwrap() };
	tokens.get(token).cloned()
}

fn generate_token() -> String {
	use std::time::{SystemTime, UNIX_EPOCH};
	use std::fmt::Write;
	let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
	let mut s = String::new();
	write!(&mut s, "{:x}", now).unwrap();
	s
}
