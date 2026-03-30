use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, SecretString};
#[derive(Debug, Clone)]
pub struct HashedPassword(SecretString);

impl PartialEq for HashedPassword {
    // New!
    fn eq(&self, other: &Self) -> bool {
        // We can use the expose_secret method to expose the SecretString
        // in a controlled manner when needed!
        self.0.expose_secret() == other.0.expose_secret() // Updated!
    }
}

impl HashedPassword {
    // Update the parse function. Note that it's now async.
    // After password validation, hash the password.
    // Using the provided helper function compute_password_hash.
    #[tracing::instrument(name = "password parsing", skip_all)]
    pub async fn parse(s: SecretString) -> Result<HashedPassword> {
        if s.expose_secret().len() < 8 {
            Err(eyre!("Password is too short!"))
        } else {
            Ok(HashedPassword(
                compute_password_hash(&s).await.map_err(|e| eyre!(e))?,
            ))
        }
    }

    // Add a parse_password_hash function.
    // To validate the format of the hash string,
    // use PasswordHash::new
    #[tracing::instrument(name = "HashedPassword Parse password hash", skip_all)]
    pub fn parse_password_hash(hash: SecretString) -> Result<HashedPassword> {
        if let Ok(hashed_string) = PasswordHash::new(hash.expose_secret()) {
            Ok(Self(SecretString::new(
                hashed_string.to_string().into_boxed_str(),
            )))
        } else {
            Err(eyre!("Failed to parse string to a HashedPassword type"))
        }
    }

    // Add a verify_raw_password function.
    // To verify the password candidate use
    // Argon2::default().verify_password.
    #[tracing::instrument(name = "Verify raw password", skip_all)]
    pub async fn verify_raw_password(&self, password_candidate: &SecretString) -> Result<()> {
        // This line retrieves the current span from the tracing context.
        // The span represents the execution context for the compute_password_hash function.
        let current_span: tracing::Span = tracing::Span::current();

        let password_hash = self.as_ref().expose_secret().to_owned();
        let password_candidate = password_candidate.expose_secret().to_owned();

        let res = tokio::task::spawn_blocking(move || {
            // This code block ensures that the operations within the closure are executed within the context of the current span.
            // This is especially useful for tracing operations that are performed in a different thread or task, such as within tokio::task::spawn_blocking.
            current_span.in_scope(|| {
                // To avoid blocking other async tasks, update this function to
                // perform hashing on a separate thread pool using
                // tokio::task::spawn_blocking.
                // Return Result<(), Box<dyn Error + Send + Sync>>.
                // Every HashedPassword instance can verify a password_candidate.
                let expected_password_hash: PasswordHash<'_> =
                    PasswordHash::new(password_hash.as_str())?;

                Argon2::default()
                    .verify_password(password_candidate.as_bytes(), &expected_password_hash)
                    .map_err(|e| e.into())
            })
        })
        .await;
        // TODO: interesting with .map_err(|e|e.into()) res? works
        // with .map_err(Box::new) res? does not work
        res?
    }
}

// Helper function to hash passwords before persisting them in storage.
//
// Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking.
// TODO: why this instrumenting is not printed?
#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: &SecretString) -> Result<SecretString> {
    let current_span: tracing::Span = tracing::Span::current();

    let password = password.expose_secret().to_owned();
    let result = tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let salt: SaltString = SaltString::generate(&mut OsRng);
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

            Ok(SecretString::new(password_hash.into_boxed_str()))
        })
    })
    .await;

    result?
}

impl AsRef<SecretString> for HashedPassword {
    // Updated!
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

#[cfg(test)]
mod tests {

    use super::HashedPassword;
    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Algorithm, Argon2, Params, PasswordHasher, Version,
    };
    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;
    use quickcheck::Gen;
    use rand::SeedableRng;
    use secrecy::{ExposeSecret, SecretString};

    #[tokio::test]
    async fn empty_string_is_rejected() {
        let password = SecretString::new("".to_string().into_boxed_str());

        assert!(HashedPassword::parse(password).await.is_err());
    }

    #[tokio::test]
    async fn string_less_than_8_characters_is_rejected() {
        let password = SecretString::new("1234567".to_string().into_boxed_str());

        assert!(HashedPassword::parse(password).await.is_err());
    }

    #[test]
    fn can_parse_valid_argon2_hash() {
        // Arrange - Create a valid Argon2 hash
        let raw_password = "TestPassword123";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );

        let hash_string = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        // Act
        let hash_password = HashedPassword::parse_password_hash(SecretString::new(
            hash_string.clone().into_boxed_str(),
        ))
        .unwrap();

        // Assert
        assert_eq!(hash_password.as_ref().expose_secret(), hash_string.as_str());
        assert!(hash_password
            .as_ref()
            .expose_secret()
            .starts_with("$argon2id$v=19$"));
    }

    #[tokio::test]
    async fn can_verify_raw_password() {
        let raw_password = SecretString::new("TestPassword123".to_string().into_boxed_str());
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None).unwrap(),
        );

        let hash_string = argon2
            .hash_password(raw_password.expose_secret().as_bytes(), &salt)
            .unwrap()
            .to_string();

        let hash_password = HashedPassword::parse_password_hash(SecretString::new(
            hash_string.clone().into_boxed_str(),
        ))
        .unwrap();

        assert_eq!(hash_password.as_ref().expose_secret(), hash_string.as_str());
        assert!(hash_password
            .as_ref()
            .expose_secret()
            .starts_with("$argon2id$v=19$"));

        // Use verify_raw_password to verify the password match
        let result = hash_password
            .verify_raw_password(&raw_password)
            .await
            .unwrap();

        assert_eq!(result, ());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub SecretString);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = g.size() as u64;
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let password: String = FakePassword(8..30).fake_with_rng(&mut rng);
            Self(SecretString::new(password.into_boxed_str()))
        }
    }

    // needs QUICKCHECK_TESTS=10
    // so let's try something new
    // #[tokio::test]
    // #[quickcheck_macros::quickcheck]
    // async fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
    //     HashedPassword::parse(valid_password.0).await.is_ok()
    // }

    #[test]
    fn valid_passwords_are_parsed_successfully() {
        fn property(valid_password: ValidPasswordFixture) -> bool {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // Assuming Password::parse_async is your asynchronous method
                HashedPassword::parse(valid_password.0).await.is_ok()
            })
        }

        // 10 tests ~ 2 seconds =)
        quickcheck::QuickCheck::new()
            .tests(10) // Set your custom number of test runs here
            .quickcheck(property as fn(ValidPasswordFixture) -> bool);
    }
    // TODO : check this
    // #[test]
    // fn async_valid_password_are_parsed_successfully_closure() {
    //     let rt = tokio::runtime::Runtime::new().unwrap();

    //     quickcheck::QuickCheck::new()
    //         .tests(500)
    //         .quickcheck(
    //             async move |valid_password: ValidPasswordFixture| -> bool {
    //                 rt.block_on(async {
    //                     // Assuming Password::parse_async is your asynchronous method
    //                     HashedPassword::parse(valid_password.0).await.is_ok()
    //                 })
    //             }
    //         );
    // }

    // use std::sync::LazyLock;
    // use tokio::runtime::Runtime;

    // // Initialize the runtime once globally
    // static GLOBAL_RT: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

    // #[test]
    // fn async_valid_password_are_parsed_successfully_lazylock() {
    //     fn property(valid_password: ValidPasswordFixture) -> bool {
    //         GLOBAL_RT.block_on(async {
    //             // Assuming Password::parse_async is your asynchronous method
    //             HashedPassword::parse(valid_password.0).await.is_ok()
    //         })
    //     }

    //     quickcheck::QuickCheck::new()
    //         .tests(10)
    //         .quickcheck(property as fn(ValidPasswordFixture) -> bool);
    // }
}
