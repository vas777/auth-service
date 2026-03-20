use crate::helpers::{get_random_email, TestApp};
use auth_service::utils::constants::JWT_COOKIE_NAME;
use serde_json::json;
use test_helpers::test_help;

#[test_help]
#[tokio::test]
async fn should_return_200_valid_token() {
    

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

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    let reqwest = json!({
        "token": auth_cookie.value().to_owned()
    });

    let response = app.post_verify_token(&reqwest).await;
    assert_eq!(response.status().as_u16(), 200)

}

#[test_help]
#[tokio::test]
async fn should_return_401_if_invalid_token() {

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

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    let mut token = auth_cookie.value().to_owned();
    // lets breakit
    token.push_str("bre.akit");
    let reqwest = json!({
        "token": token
    });

    let response = app.post_verify_token(&reqwest).await;

    assert_eq!(response.status().as_u16(), 401)
}

#[test_help]
#[tokio::test]
async fn should_return_422_if_malformed_input() {
    
    let request = json!({
        "toke": "token"
    });
    let response = app.post_verify_token(&request).await;
    assert_eq!(response.status().as_u16(), 422)
}

#[test_help]
#[tokio::test]
async fn should_return_401_if_banned_token() {
    

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

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);

    let token = auth_cookie.value().to_owned();
    let reqwest = json!({
        "token": token
    });

    let response = app.post_verify_token(&reqwest).await;
    assert_eq!(response.status().as_u16(), 401)
}
