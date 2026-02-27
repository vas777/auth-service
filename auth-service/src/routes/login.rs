use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::domain::{AuthAPIError, Email};

pub async fn login(Json(request): Json<LoginRequest>) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = Email::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;
    Ok(StatusCode::OK.into_response())
}

// TODO repeat why this are not dedicated types Email; Password?
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
