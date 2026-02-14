use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn post_signup_works() {
    let app = TestApp::new().await;

    let response = app.post_signup().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn post_login_works() {
    let app = TestApp::new().await;

    let response = app.post_login().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn post_logout_works() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn post_verify_2fa_works() {
    let app = TestApp::new().await;

    let response = app.post_verify_2fa().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn post_verify_token_works() {
    let app = TestApp::new().await;

    let response = app.post_verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
    // assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
