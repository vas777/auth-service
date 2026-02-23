use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Email, String> {
        if email.validate_email() {
            Ok(Email(email))
        } else {
            Err(format!("Not valid email: `{}`", email))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test{

    use super::Email;
    #[test]
    fn check_email() {
        assert!(Email::parse("vas@gmial.com".to_owned()).is_ok());
        assert!(Email::parse("@gmial.com".to_owned()).is_err());
        assert!(Email::parse("".to_owned()).is_err());
        let email = Email("".to_owned());
        // assert!(Email("".()).is_err());
        
    }
    
}