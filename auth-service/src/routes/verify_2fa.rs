use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode},
};

pub async fn verify_2fa(
    State(state): State<AppState>,
    Json(request): Json<Verify2FARequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let login_attempt_id = LoginAttemptId::parse(request.loginAttemptId)
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let two_fa_code =
        TwoFACode::parse(request.twoFACode).map_err(|_| AuthAPIError::InvalidCredentials)?;

    // New!
    let two_fa_code_store = state.two_fa_code_store.write().await;

    // Call `two_fa_code_store.get_code`. If the call fails
    // return a `AuthAPIError::IncorrectCredentials`.
    let code_tuple = two_fa_code_store.get_code(&email).await;

    if code_tuple.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    // TODO: bad unwrap bad
    let (stored_loginid, stored_2fa_code) = code_tuple.unwrap();
    // Validate that the `login_attempt_id` and `two_fa_code`
    // in the request body matches values in the `code_tuple`.
    // If not, return a `AuthAPIError::IncorrectCredentials`.

    if login_attempt_id != stored_loginid || two_fa_code != stored_2fa_code {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Verify2FARequest {
    email: String,
    loginAttemptId: String,
    #[serde(rename = "2FACode")]
    twoFACode: String,
}
