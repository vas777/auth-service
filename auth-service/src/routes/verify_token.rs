use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{app_state::AppState, domain::AuthAPIError, utils::auth::validate_token};

pub async fn verify_token(
    State(state): State<AppState>,
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    if let Ok(res) = state
        .banned_token_store
        .read()
        .await
        .is_banned_token(&request.token)
        .await
    {
        if res {
            return Err(AuthAPIError::IncorrectCredentials);
        }
    } else {
        return Err(AuthAPIError::UnexpectedError);
    }

    let _result = validate_token(&request.token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;
    Ok(StatusCode::OK.into_response())
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifyTokenRequest {
    token: String,
}
