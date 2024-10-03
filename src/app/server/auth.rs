use leptos::*;
use crate::app::{
    errors::{ErrorMessage, ResponseErrorTrait},
    model::{User, user::AddUserRequest, user::LoginRequest, user::DeleteUserRequest},
};


#[server(GetUsers, "/api")]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let users = retrieve_all_users().await;
    Ok(users)
}

#[server(Register, "/api")]
pub async fn add_user(add_user_request: AddUserRequest) -> Result<User, ServerFnError> {
    let new_user = add_new_user(
        add_user_request.email,
        add_user_request.password,
    )
    .await;

    match new_user {
        Some(created_user) => Ok(created_user),
        None => Err(ServerFnError::Args(String::from(
            "Error in creating user!",
        ))),
    }
}

#[server(Login, "/api")]
pub async fn login(login_request: LoginRequest) -> Result<bool, ServerFnError> {
    let user = get_user_by_mail(login_request.email).await;
    let user = match user {
        Some(u) => u,
        None => {return Err(ServerFnError::Args(String::from("User not found")))}
    };
    let verification = verify_password(login_request.password, user.password_hash);
    match verification {
        Ok(result) => Ok(result),
        Err(e) => Err(ServerFnError::Args(String::from("Error logging in!"))) 
    }
}

#[server(DeleteUser, "/api")]
pub async fn delete_user(
    delete_user_request: DeleteUserRequest,
) -> Result<User, ServerFnError> {
    let deleted_results = delete_user_entry(delete_user_request.uuid).await;
    match deleted_results {
        Ok(deleted) => {
            if let Some(deleted_user) = deleted {
                Ok(deleted_user)
            } else {
                Err(ServerFnError::Response(ErrorMessage::create(
                    UserError::UserDeleteFailure,
                )))
            }
        }
        Err(user_error) => Err(ServerFnError::Response(ErrorMessage::create(user_error))),
    }
}


cfg_if::cfg_if! {

    if #[cfg(feature = "ssr")] {

        use crate::app::db::database;
        use crate::app::errors::{ UserError };
        use chrono::{DateTime, Local};
        use uuid::Uuid;

        use argon2::{
            password_hash::{
                rand_core::OsRng,
                PasswordHash, PasswordHasher, PasswordVerifier, SaltString
            },
            Argon2
        };

        pub async fn retrieve_all_users() -> Vec<User> {
            let get_all_users_result = database::get_all_users().await;
            match get_all_users_result {
                Some(found_users) => found_users,
                None => Vec::new()
            }
        }

        pub async fn get_user_by_mail(email: String) -> Option<User> {
            database::get_user_by_mail(email).await
        }

        pub fn generate_password_hash(password: String) -> Result<String, argon2::password_hash::Error> {
            let salt = SaltString::generate(&mut OsRng);

            // Argon2 with default params (Argon2id v19)
            let argon2 = Argon2::default();

            // Hash password to PHC string ($argon2id$v=19$...)
            let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

            // Verify password against PHC string.
            //
            // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
            // `Argon2` instance.
            let parsed_hash = PasswordHash::new(&password_hash)?;
            assert!(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok());

            return Ok(password_hash);
        }

        pub fn verify_password(password: String, password_hash: String) -> Result<bool, argon2::password_hash::Error> {
            let parsed_hash = PasswordHash::new(&password_hash)?;
            Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
        }

        pub async fn add_new_user<T>(email: T, password: T)
            -> Option<User> where T: Into<String> {

            let mut buffer = Uuid::encode_buffer();
            let uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);
            
            let password_hash = generate_password_hash(password.into()).unwrap();
            // getting the current timestamp
            let current_now = Local::now();
            let current_formatted = current_now.to_string();

            let new_user = User::new(
                String::from(uuid),
                email.into(),
                password_hash,
                current_formatted
            );

            database::add_user(new_user).await
        }

        pub async fn delete_user_entry<T>(uuid: T) ->
            Result<Option<User>,UserError>
            where T: Into<String> {

            database::delete_user(uuid.into()).await
        }

    }
}