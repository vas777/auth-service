use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let result = async {
        let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;
        let token = cookie.value().to_owned();
        let _validate = validate_token(&token)
            .await
            .map_err(|_| AuthAPIError::InvalidToken)?;

        state
            .banned_token_store
            .write()
            .await
            .add_banned_token(&token)
            .await
            .map_err(|_| AuthAPIError::UnexpectedError)?;

        Ok::<_, AuthAPIError>(_validate)
    }
    .await;

    // Validate JWT token by calling `validate_token` from the auth service.
    // If the token is valid you can ignore the returned claims for now.
    // Return AuthAPIError::InvalidToken is validation fails.
    let jar = jar.remove(JWT_COOKIE_NAME);
    match result {
        Ok(_) => (jar, Ok(StatusCode::OK.into_response())),
        Err(e) => (jar, Err(e)),
    }
}
