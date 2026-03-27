use auth_service::utils::constants::JWT_COOKIE_NAME;
use reqwest::Url;
use secrecy::SecretString;

use crate::helpers::{get_random_email, TestApp};
use test_helpers::test_help;

#[test_help]
#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    let token = SecretString::new(auth_cookie.value().to_owned().into_boxed_str());

    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let result = app
        .banned_token_store
        .read()
        .await
        .is_banned_token(&token)
        .await;
    assert!(result.is_ok());
}

#[test_help]
#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400);
}

#[test_help]
#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 400)
}

#[test_help]
#[tokio::test]
async fn should_return_401_if_invalid_token() {
    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 401)
}
