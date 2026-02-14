use axum::{response::Html, routing::get, serve, Router};
use tower_http::services::ServeDir;

// #[tokio::main]
// async fn main() {
//     let assets_dir = ServeDir::new("assets");
//     let app = Router::new()
//         .fallback_service(assets_dir)
//         .route("/hello", get(hello_handler));

//     // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
//     // This is needed for Docker to work, which we will add later on.
//     // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     println!("listening on {}", listener.local_addr().unwrap());

//     axum::serve(listener, app).await.unwrap();
// }

// async fn hello_handler() -> Html<&'static str> {
//     Html("<h1>Hello, World! You made it so far and you will get even further!</h1>")
// }


use auth_service::Application;

#[tokio::main]
async fn main() {
    let app = Application::build("0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}