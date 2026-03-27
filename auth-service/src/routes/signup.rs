use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use color_eyre::eyre::{eyre, Result};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, HashedPassword, User, UserStoreError},
};

#[tracing::instrument(name = "Signup", skip_all)]
pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password = HashedPassword::parse(request.password)
        .await
        .map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    if let Err(e) = user_store.add_user(user).await {
        match e {
            UserStoreError::UserAlreadyExists => return Err(AuthAPIError::UserAlreadyExists),
            _ => return Err(AuthAPIError::UnexpectedError(eyre!(e))),
        }
    };

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

// TODO why this stayed as Strings ?
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignupRequest {
    pub email: SecretString,
    pub password: SecretString,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}
