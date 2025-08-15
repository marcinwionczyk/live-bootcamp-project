use crate::domain::{Email, Password};

// The User struct should contain 3 fields. email, which is a String;
// password, which is also a String; and requires_2fa, which is a boolean.
#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: &str, password: &str, requires_2fa: bool) -> Self {
        Self {
            email: Email(email.to_string()),
            password: Password(password.to_string()),
            requires_2fa,
        }
    }
}
