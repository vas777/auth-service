use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
    utils::auth::generate_auth_cookie,
};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // so we could continue to use '?'
    let result = async {
        let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
        let password =
            Password::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

        let user_store = state.user_store.read().await;
        user_store
            .validate_user(&email, &password)
            .await
            .map_err(|_| AuthAPIError::IncorrectCredentials)?;

        user_store
            .get_user(&email)
            .await
            .map_err(|_| AuthAPIError::IncorrectCredentials)?;

        let auth_cookie =
            generate_auth_cookie(&email).map_err(|_| AuthAPIError::UnexpectedError)?;

        Ok::<_, AuthAPIError>((auth_cookie, StatusCode::OK.into_response()))
    }
    .await;

    // Handle the single result at the very end
    match result {
        Ok((cookie, response)) => (jar.add(cookie), Ok(response)),
        Err(e) => (jar, Err(e)),
    }
}

// TODO repeat why this are not dedicated types Email; Password?
#[derive(Deserialize)]
// this will make extra fields to 422
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
