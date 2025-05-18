use std::fmt::Write;

pub fn hash_password(password: &str) -> String {
	let mut hash = 0u64;
	for b in password.bytes() {
		hash = hash.wrapping_mul(31).wrapping_add(b as u64);
	}
	let mut s = String::new();
	write!(&mut s, "{:x}", hash).unwrap();
	s
}

pub fn verify_password(password: &str, hash: &str) -> bool {
	hash_password(password) == hash
}
