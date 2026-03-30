use auth_service::{
    app_state::{AppState, BannedTokens},
    domain::Email,
    get_postgres_pool, get_redis_client,
    services::{
        MockEmailClient, PostgresUserStore, PostmarkEmailClient, RedisBannedTokenStore,
        RedisTwoFACodeStore,
    },
    utils::{
        constants::{prod, DATABASE_URL, POSTMARK_AUTH_TOKEN, POSTMARK_EMAIL, REDIS_HOST_NAME},
        tracing::init_tracing,
    },
    Application,
};
use reqwest::Client;
use secrecy::SecretString;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing().expect("Failed to initialize tracing");
    // pretty_env_logger::init();

    let pg_pool = Arc::new(RwLock::new(PostgresUserStore::new(
        configure_postgresql().await,
    )));

    let redis = Arc::new(RwLock::new(configure_redis()));

    let banned_token_store: BannedTokens =
        Arc::new(RwLock::new(RedisBannedTokenStore::new(redis.clone())));

    let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis.clone())));
    // let email_client = Arc::new(MockEmailClient);
    let email_client = Arc::new(configure_postmark_email_client());
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

fn configure_postmark_email_client() -> PostmarkEmailClient {
    let http_client = Client::builder()
        .timeout(prod::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(
        prod::email_client::BASE_URL.to_owned(),
        Email::parse(POSTMARK_EMAIL.clone()).unwrap(),
        POSTMARK_AUTH_TOKEN.to_owned(),
        http_client,
    )
}
