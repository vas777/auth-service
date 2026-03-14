use sqlx::PgPool;

use crate::domain::{
    {UserStore, UserStoreError},
    Email, HashedPassword, User,
};

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

    // TODO: Implement all required methods. Note that you will need to make SQL queries against our PostgreSQL instance inside these methods. Ensure to parse the password_hash.
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>{todo!()}
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>{todo!()}
    async fn validate_user(&self, email: &Email, raw_password: &str) -> Result<(), UserStoreError>{todo!()}
}