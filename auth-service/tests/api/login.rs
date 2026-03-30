use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME,
};
use secrecy::SecretString;
use serde::Serialize;
use test_helpers::test_help;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

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
    }
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
    // should not contact mock
    // therefore expect 0
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&app.email_server)
        .await;

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
    let email = Email::parse(SecretString::new(random_email.clone().into_boxed_str())).unwrap();
    let signup_body = serde_json::json!({
        "email": random_email.clone(),
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    // Define an expectation for the mock server
    Mock::given(path("/email")) // Expect an HTTP request to the "/email" path
        .and(method("POST")) // Expect the HTTP method to be POST
        .respond_with(ResponseTemplate::new(200)) // Respond with an HTTP 200 OK status
        .expect(1) // Expect this request to be made exactly once
        .mount(&app.email_server) // Mount this expectation on the mock email server
        .await; // Await the asynchronous operation to ensure the mock server is set up before proceeding

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

    let two_fa_code_store = app.two_fa_code_store.read().await;

    let code_tuple = two_fa_code_store
        .get_code(&email)
        .await
        .expect("Failed to get 2FA code");

    assert_eq!(code_tuple.0.as_ref(), response.login_attempt_id);
}
