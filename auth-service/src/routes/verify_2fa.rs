use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie,
};

pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let (Ok(email), Ok(login_attempt_id), Ok(two_fa_code)) = (
        Email::parse(request.email),
        LoginAttemptId::parse(request.login_attempt_id),
        TwoFACode::parse(request.two_fa_code),
    ) else {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    };

    let mut two_fa_code_store = state.two_fa_code_store.write().await;

    // Call `two_fa_code_store.get_code`. If the call fails
    // return a `AuthAPIError::IncorrectCredentials`.
    let Ok((stored_loginid, stored_2fa_code)) = two_fa_code_store.get_code(&email).await else {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    };

    // Validate that the `login_attempt_id` and `two_fa_code`
    // in the request body matches values in the `code_tuple`.
    // If not, return a `AuthAPIError::IncorrectCredentials`.
    if login_attempt_id != stored_loginid || two_fa_code != stored_2fa_code {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    if two_fa_code_store.remove_code(&email).await.is_err() {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    let auth_cookie = match generate_auth_cookie(&email) {
        Ok(cookie) => cookie,
        Err(_) => return (jar, Err(AuthAPIError::UnexpectedError)),
    };

    let updated_jar = jar.add(auth_cookie);

    (updated_jar, Ok(StatusCode::OK.into_response()))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String,
}
