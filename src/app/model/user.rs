use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct User {
    pub uuid: String,
    #[validate(email)]
    pub email: String,
    pub password_hash: String,
    pub joined_date: String,
    pub name: String,
    pub last_name: String,
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
            name: String::new(),
            last_name: String::new(),
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8), )]
    pub password: String,
}

impl RegisterRequest {
    pub fn new(email: String, password: String) -> RegisterRequest {
        RegisterRequest {
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
    pub fn new(email: String, password: String) -> LoginRequest {
        LoginRequest {
            email,
            password,
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct DeleteUserRequest {
    pub uuid: String,
}

impl DeleteUserRequest {
    pub fn new(uuid: String) -> DeleteUserRequest {
        DeleteUserRequest { uuid }
    }
}