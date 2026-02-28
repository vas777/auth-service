use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.read().await;
    if user_store.validate_user(&email, &password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    let u = user_store.get_user(&email).await;
    // TODO why this is necessary? if validate does get_user I will fail if
    // if email is wrong
    // and tests should catch if that logic changes.
    if u.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    Ok(StatusCode::OK.into_response())
}

// TODO repeat why this are not dedicated types Email; Password?
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
