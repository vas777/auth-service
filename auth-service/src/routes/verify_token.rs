use crate::{app_state::AppState, domain::AuthAPIError, utils::auth::validate_token};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use color_eyre::eyre::{Result};
use serde::Deserialize;
use secrecy::{SecretString};

#[tracing::instrument(name = "Verifying token", skip_all)]
pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = SecretString::new(request.token.into_boxed_str()); 
    let result = state
        .banned_token_store
        .read()
        .await
        .is_banned_token(&token)
        .await;

    match result {
        Ok(verdict) => {
            if verdict {
                return Err(AuthAPIError::IncorrectCredentials);
            }
        }
        Err(e) => return Err(AuthAPIError::UnexpectedError(e.into())),
    }

    let _result = validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;
    Ok(StatusCode::OK.into_response())
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifyTokenRequest {
    token: String,
}
