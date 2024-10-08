use crate::app::{
    errors::{ErrorMessage, ResponseErrorTrait},
    model::{user::DeleteUserRequest, user::LoginRequest, user::RegisterRequest, User},
};
use leptos::*;
use serde::{Deserialize, Serialize};
use leptos::logging::log;



#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time in seconds
    pub iat: usize,
}

#[server(Register, "/api")]
pub async fn register(add_user_request: RegisterRequest) -> Result<User, ServerFnError> {
    let new_user = add_new_user(add_user_request.email, add_user_request.password).await;

    match new_user {
        Ok(created_user) => Ok(created_user),
        Err(_) => Err(ServerFnError::Args(String::from("Error in creating user!"))),
    }
}



#[server]
pub async fn actix_extract() -> Result<String, ServerFnError> {
    use actix_web::HttpRequest;
    let req = use_context::<HttpRequest>();
    let cookie = req.unwrap().cookie("auth_token").unwrap();
    let cookie_name = cookie.name();
    let cookie_value = cookie.value();
    Ok("successfully read cookie!")
}

#[server(Login, "/api")]
pub async fn login(
    login_request: LoginRequest,
) -> Result<String, ServerFnError> {

    use actix_web::{cookie::{time::Duration, Cookie, SameSite},  http::{header::{self, HeaderValue}, StatusCode}, HttpRequest};
    use leptos_actix::{ResponseOptions};

    let user = get_user_by_mail(login_request.email).await;
    let user = match user {
        Some(u) => u,
        None => return Err(ServerFnError::Args(String::from("User not found"))),
    };
    let verification = verify_password(login_request.password, user.password_hash).await;
    let verification = match verification {
        Ok(result) => result,
        Err(_) => return Err(ServerFnError::Args(String::from("Error logging in!"))),
    };
    if verification {
        match generate_jwt(Uuid::parse_str(&user.uuid).unwrap()).await {
            Ok(token) => {
                let cookie = Cookie::build("auth_token", &token)
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Lax)
                    .path("/")
                    .max_age(Duration::days(30))
                    .finish();
                let response = expect_context::<ResponseOptions>();
                response.set_status(StatusCode::OK);
                if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
                    response.insert_header(header::SET_COOKIE, cookie);
                }
                Ok(token)
            }
            Err(_) => Err(ServerFnError::Args(String::from("Error logging in!"))),
        }
    } else {
        Err(ServerFnError::Args(String::from("Error logging in!")))
    }
}

#[server(DeleteUser, "/api")]
pub async fn delete_user(delete_user_request: DeleteUserRequest) -> Result<User, ServerFnError> {
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
        use chrono::Local;
        use uuid::Uuid;
        use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
        use std::env;

        use argon2::{
            password_hash::{
                rand_core::OsRng,
                PasswordHash, PasswordHasher, PasswordVerifier, SaltString
            },
            Argon2
        };

        async fn get_user_by_mail(email: String) -> Option<User> {
            database::get_user_by_mail(email).await
        }

        async fn generate_password_hash(password: String) -> Result<String, argon2::password_hash::Error> {
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

        async fn verify_password(password: String, password_hash: String) -> Result<bool, argon2::password_hash::Error> {
            let parsed_hash = PasswordHash::new(&password_hash)?;
            Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
        }

        async fn generate_jwt(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
            let claims = JWTClaims {
                sub: user_id.to_string(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
                iat: chrono::Utc::now().timestamp() as usize,
            };
            let secret = env::var("JWT_KEY").unwrap();
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()));
            logging::log!("UUID: {:?}\nToken: {:?}\n decoded: {:?}", user_id, token, validate_jwt(token.as_ref().unwrap().as_str()).await);
            token
        }

        async fn validate_jwt(token: &str) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
            let secret = env::var("JWT_KEY").unwrap();
            let decoded = decode::<JWTClaims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())?;
            Ok(decoded.claims)
        }

        async fn add_new_user<T>(email: T, password: T)
            -> Result<User, UserError> where T: Into<String> {

            let uuid = Uuid::new_v4();

            let password_hash = match generate_password_hash(password.into()).await {
                Ok(hash) => hash,
                Err(_) => { return Err(UserError::UserCreationFailure); },
            };
            // getting the current timestamp
            let current_now = Local::now();
            let current_formatted = current_now.to_string();

            let new_user = User::new(
                uuid.into(),
                email.into(),
                password_hash,
                current_formatted
            );

            match database::add_user(new_user).await {
                Some(user) => Ok(user),
                None => Err(UserError::UserCreationFailure),
            }
        }

        async fn delete_user_entry<T>(uuid: T) ->
            Result<Option<User>,UserError>
            where T: Into<String> {

            database::delete_user(uuid.into()).await
        }

    }
}
