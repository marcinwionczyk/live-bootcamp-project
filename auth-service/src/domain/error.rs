use crate::domain::UserStoreError;

#[derive(Debug, PartialEq)]
pub enum AuthAPIError {
    UserAlreadyExists,
    IncorrectCredentials,
    UnexpectedError,
    Unauthorized,
    UserNotFound,
    UnprocessableContent,
}

impl From<UserStoreError> for AuthAPIError {
    fn from(e: UserStoreError) -> Self {
        match e {
            UserStoreError::UserAlreadyExists => AuthAPIError::UserAlreadyExists,
            UserStoreError::InvalidCredentials => AuthAPIError::IncorrectCredentials,
            UserStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
            UserStoreError::UserNotFound => AuthAPIError::UserNotFound,
        }
    }
}
