#[derive(Debug)]

pub enum ResponseError {
    UserNotFound,
    UserUpdateFailure,
    UserCreationFailure,
    UserDeleteFailure,
}

pub type ErrorMessage = String;

pub trait ResponseErrorTrait {
    fn create(user_error: ResponseError) -> ErrorMessage;
}

impl ResponseErrorTrait for ErrorMessage {
    fn create(user_error: ResponseError) -> ErrorMessage {
        match user_error {
            ResponseError::UserNotFound => ErrorMessage::from("User not found"),
            ResponseError::UserUpdateFailure => ErrorMessage::from("failed to update user"),
            ResponseError::UserCreationFailure => ErrorMessage::from("failed to create user"),
            ResponseError::UserDeleteFailure => ErrorMessage::from("failed to delete user"),
        }
    }
}