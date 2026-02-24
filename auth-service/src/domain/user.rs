use crate::domain::{Email, Password};

// The User struct should contain 3 fields. email, which is a String;
// password, which is also a String; and requires_2fa, which is a boolean.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        User {
            email: email,
            password: password,
            requires_2fa: requires_2fa,
        }
    }
}
