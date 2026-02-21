use validator::ValidateEmail;

#[derive(Debug)]
pub struct Email(String);

impl Email {
    fn parse(&self)-> Result<(),String> {
        if !self.0.validate_email() {
            return Err(format!("Not valid email {}", self.0))
        }
        Ok(())
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
#[test]
fn check_email() {
    let email = Email("vas@gmial.com".to_owned());
    assert_eq!(email.parse(), Ok(()));
    println!("{email:?}");
    let email = Email("vasgmial.com".to_owned());
    assert_eq!(email.parse(), Err("Not valid email vasgmial.com".to_owned()));
    let email = Email("vasgmial.com".to_owned());
    assert_eq!(email.parse(), Err("Not valid email vasgmial.com".to_owned()));
    // let email = Email("vasgmial.com".to_owned());
    // assert_eq!(email.parse(), Ok(()));
}
}