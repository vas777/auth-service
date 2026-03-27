use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, SecretString};
use std::hash::Hash;
use validator::ValidateEmail;
#[derive(Debug, Clone)]
pub struct Email(SecretString);

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

// New!
impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

// New!
impl Eq for Email {}

impl Email {
    #[tracing::instrument(name = "email parsing", skip_all, err(Debug))]
    pub fn parse(email: SecretString) -> Result<Email> {
        if email.expose_secret().validate_email() {
            Ok(Self(email))
        } else {
            Err(eyre!(format!(
                "Not valid email: `{}`",
                email.expose_secret()
            )))
        }
    }
}

impl AsRef<SecretString> for Email {
    fn as_ref(&self) -> &SecretString {
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
    use secrecy::SecretString;

    #[test]
    fn check_email() {
        let email = SecretString::new("vas@gmail.com".to_owned().into_boxed_str());
        assert!(Email::parse(email).is_ok());

        let email = SecretString::new("vasgmial.com".to_owned().into_boxed_str());
        assert!(Email::parse(email).is_err());

        let email = SecretString::new("".to_owned().into_boxed_str());
        assert!(Email::parse(email).is_err());

        let email = SecretString::new("@".to_owned().into_boxed_str());
        assert!(Email::parse(email).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = u64::arbitrary(g);
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let email: String = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(SecretString::new(valid_email.0.into_boxed_str())).is_ok()
    }
}
