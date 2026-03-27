use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::Result;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, HashedPassword, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie,
};

#[tracing::instrument(name = "Login", skip_all)]
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    if HashedPassword::parse(request.password.clone())
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    };

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.read().await;

    if user_store
        .validate_user(&email, &request.password)
        .await
        .is_err()
    {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

#[tracing::instrument(name = "Handling 2FA", skip_all)]
async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();
    // Return a TwoFactorAuthResponse. The message should be "2FA required".
    // The login attempt ID should be "123456". We will replace this hard-coded login attempt ID soon!

    // Store the ID and code in our 2FA code store.
    // Return `AuthAPIError::UnexpectedError` if the operation fails
    if let Err(e) = state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

    // send 2FA code via the email client. Return `AuthAPIError::UnexpectedError` if the operation fails.
    if let Err(e) = state
        .email_client
        .write()
        .await
        .send_email(email, "2FA code", two_fa_code.as_ref())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e)));
    }

    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_owned(),
    }));

    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

#[tracing::instrument(name = "Handling NO 2FA", skip_all)]
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(e) => return (jar, Err(AuthAPIError::UnexpectedError(e))),
    };

    let updated_jar = jar.add(auth_cookie);

    (
        updated_jar,
        Ok((StatusCode::OK, Json(LoginResponse::RegularAuth))),
    )
}

// TODO repeat why this are not dedicated types Email; Password?
#[derive(Deserialize)]
// this will make extra fields to 422
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    pub email: SecretString,
    pub password: SecretString,
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
