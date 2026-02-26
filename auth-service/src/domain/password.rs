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

    use fake::faker::internet::en::Password as TestPassword;
    use fake::Fake;
    use quickcheck::Gen;
    use rand::SeedableRng;

    #[test]
    fn check_password() {
        assert!(Password::parse("".to_owned()).is_err());
        assert!(Password::parse("123".to_owned()).is_err());
        assert!(Password::parse("12345678".to_owned()).is_ok());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = u64::arbitrary(g);
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let password = TestPassword(8..30).fake_with_rng(&mut rng);
            Self(password)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_password_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        Password::parse(valid_password.0).is_ok()
    }

}
