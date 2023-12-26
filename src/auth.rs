use regex::Regex;

pub(crate) fn is_valid_password(password: String) -> bool {
    password.len() >= 6
}

pub fn is_valid_email(email: String) -> bool {
    Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
        .unwrap()
        .is_match(&email)
}
