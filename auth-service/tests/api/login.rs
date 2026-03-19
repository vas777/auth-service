use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::{Email, TwoFACode},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
};
use serde::Serialize;
use test_helpers::test_help;

#[test_help]
#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    // TODO with this leads to sig abort ?
    // 
    // let mut app = TestApp::new().await;
    // let mut app = TestApp::new().await;


    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({
            "pas": "password123",
            "ema": random_email,
            "req": true
        }),
        serde_json::json!({
            "email": "vas@gmail.com" ,
            "password": "password123",
            "req": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    };
}

#[test_help]
#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    
    // TODO wouldn't it be better if we had structure here ?
    #[derive(Serialize, Debug)]
    struct LoginRequest {
        email: String,
        password: String,
    }

    let test_cases = [
        LoginRequest {
            email: "myemail".to_owned(),
            password: "password".to_owned(),
        },
        LoginRequest {
            email: get_random_email(),
            password: "1234".to_owned(),
        },
        LoginRequest {
            email: "email".to_owned(),
            password: "12345678".to_owned(),
        },
    ];

    for test_case in test_cases {
        let response = app.post_login(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed with {:?}",
            test_case
        )
    }
}

#[test_help]
#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.
    

    let email = String::from("mycorrect@email.com");
    let password = String::from("mylongcorrectvalidpass");

    let to_login = serde_json::json!({
        "email": email.clone(),
        "password": password.clone(),
    });
    let response = app.post_login(&to_login).await;

    assert_eq!(
        response.status().as_u16(),
        401,
        "Failed with {:?}",
        to_login
    )
}

#[test_help]
#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    

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

    assert!(!auth_cookie.value().is_empty());
}

#[test_help]
#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    

    let random_email = get_random_email();
    let email = Email::parse(random_email.clone()).unwrap();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);
    let response = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert!(response.message.eq("2FA required"));
    assert!(TwoFACode::parse(response.login_attempt_id).is_ok());

    assert!(app
        .two_fa_code_store
        .read()
        .await
        .get_code(&email)
        .await
        .is_ok());
}
