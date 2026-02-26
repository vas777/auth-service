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
mod test {

    use super::Email;

    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::Gen;
    use rand::SeedableRng;

    #[test]
    fn check_email() {
        assert_eq!(
            Email::parse("vas@gmail.com".to_owned()),
            Ok(Email("vas@gmail.com".to_owned()))
        );
        assert_eq!(Email::parse("vasgmial.com".to_owned()), Err("Not valid email: `vasgmial.com`".to_owned()));
        assert_eq!(Email::parse("".to_owned()), Err("Not valid email: ``".to_owned()));
        assert_eq!(Email::parse("@".to_owned()), Err("Not valid email: `@`".to_owned()));
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = u64::arbitrary(g);
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(valid_email.0).is_ok()
    }

}
