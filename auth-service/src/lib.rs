use axum::{Router, serve::Serve};
use axum::{response::Html, routing::get};
use tokio::net::TcpListener;
use std::error::Error;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {

    async fn hello_handler() -> Html<&'static str> {
        Html("<h1>Hello, World! You made it so far and you will get even further!</h1>")
    } 

    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let router =  Router::new()
        .route("/", get(crate::hello_handler));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        todo!()
    }



    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}