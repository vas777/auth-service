use auth_service::{
    Application, app_state::{AppState, BannedTokens, UserStoreType}, get_postgres_pool, get_redis_client, services::{
        HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore, MockEmailClient,
        PostgresUserStore,
    }, utils::constants::{DATABASE_URL, REDIS_HOST_NAME, prod}
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let pg_pool = Arc::new(RwLock::new(PostgresUserStore::new(
        configure_postgresql().await,
    )));
    let banned_token_store: BannedTokens =
        Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(MockEmailClient));
    let app_state = AppState::new(pg_pool, banned_token_store, two_fa_code_store, email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database!
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}
