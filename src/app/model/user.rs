use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct User {
    pub uuid: String,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    pub joined_date: String,
}

impl User {
    pub fn new(
        uuid: String,
        email: String,
        password_hash: String,
        joined_date: String,
    ) -> User {
        User {
            uuid,
            email,
            password_hash,
            joined_date,
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct AddUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8), )]
    pub password: String,
}

impl AddUserRequest {
    pub fn new(email: String, password: String) -> AddUserRequest {
        AddUserRequest {
            email,
            password,
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8), )]
    pub password: String,
}

impl LoginRequest {
    pub fn new(email: String, password: String) -> AddUserRequest {
        AddUserRequest {
            email,
            password,
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct DeleteUserRequest {
    #[validate(length(min = 1, message = "id is required"))]
    pub uuid: String,
}

impl DeleteUserRequest {
    pub fn new(uuid: String) -> DeleteUserRequest {
        DeleteUserRequest { uuid }
    }
}