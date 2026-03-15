use crate::domain::{Email, HashedPassword};

// The User struct should contain 3 fields. email, which is a String;
// password_hash, which is also a String; and requires_2fa, which is a boolean.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub email: Email,
    pub password_hash: HashedPassword,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password_hash: HashedPassword, requires_2fa: bool) -> Self {
        User {
            email: email,
            password_hash: password_hash,
            requires_2fa: requires_2fa,
        }
    }
}
