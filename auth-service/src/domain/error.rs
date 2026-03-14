pub enum AuthAPIError {
    UnexpectedError,      // 500
    UserAlreadyExists,    // 409
    IncorrectCredentials, // 401
    InvalidCredentials,   // 400
    MissingToken,
    InvalidToken,
}
