use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Html, routing::get, routing::post};
use axum::{serve::Serve, Router};
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub mod routes;

use crate::routes::{login, logout, signup, verify_2fa, verify_token};

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let asset_dir =
            ServeDir::new("assets").not_found_service(ServeDir::new("assets/index.html"));
        let router = Router::new()
            .fallback_service(asset_dir)
            // TODO what about root / ?
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa)) // Example route
            .route("/verify-token", post(verify_token)); // Example route

        // TODO ask: async before || and after || ? Is there difference ?
        // .route("/", get(  || async {
        //     Html("<h1>Hello, World! You made it so far and you will get even further!</h1>")
        // }));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application {
            server: server,
            address: address,
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
