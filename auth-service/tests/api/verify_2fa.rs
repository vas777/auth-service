use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::{Email, TwoFACode},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
};

use uuid::Uuid;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "email": "vas@gmail.com" ,
            "loginAttemptId": "password123",
            "2FACode": "true1234",
            "notdefined": "extra"
        }),
        serde_json::json!({
            "email": "vas@gmail.com" ,
            "loginAttemptId": "password123",
            "2FACode": true
        }),
        serde_json::json!({
            "password": "password123",
            "2FACode": true
        }),
        serde_json::json!({
            "email": random_email,
            "2FACode": "123456"
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": true
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({
            "login": "password123",
            "ema": random_email,
            "req": true
        }),
    ];

    for test_case in test_cases.iter() {
        println!("{:?}", test_case);
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let test_cases = [
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "not-uuid-at-all",
            "2FACode": "123456",
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": "not-uuid-at-all",
            "2FACode": "123456",
        }),
        serde_json::json!({
            "email": random_email,
            "loginAttemptId": Uuid::new_v4(),
            "2FACode": "1",
        }),
        serde_json::json!({
            "email": "bademaildotcom",
            "loginAttemptId": Uuid::new_v4(),
            "2FACode": "123456",
        }),
    ];

    for test_case in test_cases.iter() {
        println!("{:?}", test_case);
        let response = app.post_verify_2fa(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
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

    let verify_2fa_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": Uuid::new_v4(),
        "2FACode": "000000"
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail.
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email.clone(),
        "password": "password123",
    });

    let response1 = app.post_login(&login_body).await;

    let (old_id, old_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let response2 = app.post_login(&login_body).await;

    assert_eq!(response2.status().as_u16(), 206);
    let response = response1
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert!(response.message.eq("2FA required"));
    assert!(TwoFACode::parse(response.login_attempt_id.clone()).is_ok());

    let verify_2fa_body = serde_json::json!({
        "email": random_email.clone(),
        "loginAttemptId": old_id.as_ref().to_owned(),
        "2FACode": old_code.as_ref().to_owned()
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    // Make sure to assert the auth cookie gets set
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email.clone(),
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    let (id, code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let response = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");
    assert!(response.message.eq("2FA required"));
    assert!(TwoFACode::parse(response.login_attempt_id.clone()).is_ok());

    let verify_2fa_body = serde_json::json!({
        "email": random_email.clone(),
        "loginAttemptId": id.as_ref().to_owned(),
        "2FACode": code.as_ref().to_owned()
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 200);

    response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email.clone(),
        "password": "password123",
    });

    app.post_login(&login_body).await;

    let (id, code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone()).unwrap())
        .await
        .unwrap();

    let verify_2fa_body = serde_json::json!({
        "email": random_email.clone(),
        "loginAttemptId": id.as_ref().to_owned(),
        "2FACode": code.as_ref().to_owned()
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_verify_2fa(&verify_2fa_body).await;
    assert_eq!(response.status().as_u16(), 401);

}
