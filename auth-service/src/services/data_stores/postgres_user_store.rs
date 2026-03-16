use sqlx::PgPool;

use crate::domain::{Email, HashedPassword, User, UserStore, UserStoreError};

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
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // just to be sure
        let _password_hash =
            HashedPassword::parse_password_hash(user.password_hash.as_ref().to_owned())
                .map_err(|_| UserStoreError::UnexpectedError)?;

        sqlx::query!(
            // language=PostgreSQL
            r#"
            insert into "users"(email, password_hash, requires_2fa)
            values ($1, $2, $3)
        "#,
            user.email.as_ref().to_owned(),
            user.password_hash.as_ref().to_owned(),
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| match e {
            sqlx::Error::Database(dbe) if dbe.constraint() == Some("user_username_key") => {
                UserStoreError::UserAlreadyExists
            }
            _ => UserStoreError::UnexpectedError,
        })
    }
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let maybe_user = sqlx::query!(
            // language=PostgreSQL
            r#"select * from users where email = $1"#,
            email.as_ref().to_owned()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| UserStoreError::UnexpectedError);

        if let Ok(user) = maybe_user {
            let email = Email::parse(user.email).map_err(|_| UserStoreError::UnexpectedError)?;
            let password_hash = HashedPassword::parse_password_hash(user.password_hash)
                .map_err(|_| UserStoreError::UnexpectedError)?;
            Ok(User::new(email, password_hash, user.requires_2fa))
        } else {
            Err(UserStoreError::UnexpectedError)
        }
    }
    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;

        user.password_hash
            .verify_raw_password(raw_password)
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)?;

        Ok(())
    }
}
