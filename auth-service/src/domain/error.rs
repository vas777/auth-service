pub enum AuthAPIError {
    UserAlreadyExists,    // 409
    IncorrectCredentials, // 401
    InvalidCredentials,   // 400
    UnexpectedError,      // 500
}
