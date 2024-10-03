// for User error

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("member not found")]
    UserNotFound,
    #[error("failed to update member")]
    UserUpdateFailure,
    #[error("failed to create member")]
    UserCreationFailure,
    #[error("failed to delete member")]
    UserDeleteFailure,
}

pub type ErrorMessage = String;

pub trait ResponseErrorTrait {
    fn create(User_error: UserError) -> ErrorMessage;
}

impl ResponseErrorTrait for ErrorMessage {
    fn create(User_error: UserError) -> ErrorMessage {
        match User_error {
            UserError::UserNotFound => ErrorMessage::from("User not found"),
            UserError::UserUpdateFailure => ErrorMessage::from("failed to update user"),
            UserError::UserCreationFailure => ErrorMessage::from("failed to create user"),
            UserError::UserDeleteFailure => ErrorMessage::from("failed to delete user"),
        }
    }
}