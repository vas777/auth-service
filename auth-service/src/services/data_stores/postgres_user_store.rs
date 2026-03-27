use sqlx::PgPool;
use secrecy::{ExposeSecret, SecretString};
use crate::domain::{Email, HashedPassword, User, UserStore, UserStoreError};
use color_eyre::eyre::{eyre, Result};

#[derive(Clone)]
pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    // Implement all required methods.
    // Note that you will need to make SQL queries against our PostgreSQL instance inside these methods.
    // Ensure to parse the password_hash.
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // just to be sure
        // TODO  check this
        // let _password_hash =
        //     HashedPassword::parse_password_hash(user.password_hash.as_ref().to_owned())
        //         .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

        sqlx::query!(
            // language=PostgreSQL
            r#"
            insert into "users"(email, password_hash, requires_2fa)
            values ($1, $2, $3)
        "#,
            user.email.as_ref().expose_secret(),
            user.password_hash.as_ref().expose_secret(),
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("users_pkey") => {
                UserStoreError::UserAlreadyExists
            }
            _ => UserStoreError::UnexpectedError(e.into()),
        })?;

        Ok(())
    }

    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let user = sqlx::query!(
            // language=PostgreSQL
            r#"select * from users where email = $1"#,
            email.as_ref().expose_secret()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => UserStoreError::UserNotFound,
            _ => UserStoreError::UnexpectedError(eyre!(e)),
        })?;

        let email =
            Email::parse(SecretString::new(user.email.into_boxed_str())).map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?;
        let password_hash = HashedPassword::parse_password_hash(SecretString::new(user.password_hash.into_boxed_str()))
            .map_err(|e| UserStoreError::UnexpectedError(eyre!(e)))?;
        Ok(User::new(email, password_hash, user.requires_2fa))

        // TODO : map and ok_or ?
        //         sqlx::query!(
        //     r#"
        //     SELECT email, password_hash, requires_2fa
        //     FROM users
        //     WHERE email = $1
        //     "#,
        //     email.as_ref()
        // )
        // .fetch_optional(&self.pool)
        // .await
        // .map_err(|e| UserStoreError::UnexpectedError(e.into()))?
        // .map(|row| {
        //     Ok(User {
        //       email: Email::parse(row.email)
        //           .map_err(|e| UserStoreError::UnexpectedError(eyre!
        //           (e)))?, // Updated!
        //       password: HashedPassword::parse_password_hash(
        //           row.password_hash)
        //           .map_err(|e| UserStoreError::UnexpectedError(eyre!
        //           (e)))?, // Updated!
        //       requires_2fa: row.requires_2fa,
        //     })
        // })
        // .ok_or(UserStoreError::UserNotFound)?
    }

    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate_user(&self, email: &Email, raw_password: &SecretString) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        user.password_hash
            .verify_raw_password(raw_password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)?;

        Ok(())
    }
}
