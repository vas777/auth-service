#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Password, String> {
        if password.len() < 8 {
            Err(format!("Password is too short!"))
        } else {
            Ok(Password(password))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::Password;
    #[test]
    fn check_password() {
        assert!(Password::parse("".to_owned()).is_err());
        assert!(Password::parse("123".to_owned()).is_err());
        assert!(Password::parse("12345678".to_owned()).is_ok());
    }
}
