
use leptos::{prelude::*, task::spawn_local};
use leptos::logging::log;
use leptos::ev::{self, MouseEvent};
use validator::Validate;
use serde::{Deserialize, Serialize};

use crate::app::model::{
    user::LoginRequest,
    user::RegisterRequest,
    user::AuthenticateRequest,
    User
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time in seconds
    pub iat: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CurrentModal {
    Login,
    Register,
    None,
}

stylance::import_style!(style, "auth.module.scss");

#[component]
pub fn AuthForm(
    show_modal: ReadSignal<bool>,
    set_show_modal: WriteSignal<bool>,
) -> impl IntoView {

    let (current_modal, set_current_modal) = signal(CurrentModal::Login);

    // let on_back_pressed = move |_| {
    //     set_show_modal(false);
    // };

    let (_user, set_user) = signal::<Option<User>>(None);
    spawn_local(initial_auth(set_user));

    view! {
        <Show when = move || { show_modal() }>
            <div class=style::back>
                <Show when = move || { current_modal() == CurrentModal::Register }>
                    <SignUpForm set_current_modal set_show_modal set_user/>
                </Show>
                <Show when = move || { current_modal() == CurrentModal::Login }>
                    <SignInForm set_current_modal set_show_modal set_user/>
                </Show>
            </div>
        </Show>
    }
}

async fn initial_auth(set_user: WriteSignal<Option<User>>) {
    match authenticate(None).await {
        Ok(user) => {
            set_user(Some(user));
        },
        Err(e) => {
            log!("Error: {:?}", e);
        }
    };
} 

#[component]
pub fn SignInForm(
    set_current_modal: WriteSignal<CurrentModal>,
    set_show_modal: WriteSignal<bool>,
    set_user: WriteSignal<Option<User>>
) -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let (error_message, set_error_message) = signal(String::new());
    let (if_error, set_if_error) = signal(false);

    let on_register_pressed = move |_| {
        set_current_modal(CurrentModal::Register);
    };

    let on_login = move |_| {
        let login_request = LoginRequest::new(email(), password());
        let is_valid = login_request.validate();
        log!{"starting login process..."}
        match is_valid {
            Ok(_) => {
                spawn_local(async move {
                    log!{"spawned login process..."}
                    let login_result = sign_in(login_request).await;
                    log!{"finished login process..."}

                    match login_result {
                        Ok(user) => {
                            log! {"success"};
                            set_show_modal(false);
                            set_user(Some(user));
                        }
                        Err(e) => {
                            log!("Error {:?}", e);
                            set_error_message(format! {"Error adding {:?}", e});
                        }
                    }
                });
            }
            Err(_) => {
                set_if_error(true);
                set_error_message(String::from("All fields are required!"));
            }
        }
    };

    let on_enter = move |e: ev::KeyboardEvent| {
        if e.key() == "Enter" {
            on_login(MouseEvent::new("").unwrap());
        }
    };
    
    view! {
        <div class=style::container>
            <input type="email" placeholder="E-Mail"
                value=email
                on:input=move |e| {
                    set_email(event_target_value(&e));
                }
                class=style::input
            />
            <input type="password" placeholder="Passwort"
                value=password
                on:input=move |e| {
                    set_password(event_target_value(&e));
                }
                on:keydown = on_enter
                class=style::input
            />
            <span class=style::error_label>
                <Show when = move || { if_error() }>
                    {error_message()}
                </Show>
            </span>
            <button on:click=on_login class=style::button>"Login"</button>
            <a class=style::link on:click=on_register_pressed>"Neues Konto erstellen"</a>
        </div>
    }
}

#[server(SignIn, "/api")]
pub async fn sign_in(
    login_request: LoginRequest,
) -> Result<User, ServerFnError> {
    use actix_web::{cookie::{time::Duration, Cookie, SameSite},  http::{header::{self, HeaderValue}, StatusCode}};
    use leptos_actix::ResponseOptions;

    let user = get_user_by_mail(login_request.email).await;
    let user = match user {
        Some(u) => u,
        None => return Err(ServerFnError::Args(String::from("User not found"))),
    };

    let verification = verify_password(login_request.password.to_owned(), user.password_hash.to_owned()).await;
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
                Ok(user)
            }
            Err(_) => Err(ServerFnError::Args(String::from("Error logging in!"))),
        }
    } else {
        Err(ServerFnError::Args(String::from("Error logging in!")))
    }
}


#[component]
pub fn SignUpForm(
    set_current_modal: WriteSignal<CurrentModal>,
    set_show_modal: WriteSignal<bool>,
    set_user: WriteSignal<Option<User>>,
) -> impl IntoView {

    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());

    let (error_message, set_error_message) = signal(String::new());
    let (if_error, set_if_error) = signal(false);

    let on_login_pressed = move |_| {
        set_current_modal(CurrentModal::Login);
    };

    let on_register = move |_| {
        let register_request = RegisterRequest::new(email(), password());
        let is_valid = register_request.validate();

        match is_valid {
            Ok(_) => {
                spawn_local(async move {
                    let register_result = sign_up(register_request).await;

                    match register_result {
                        Ok(user) => {
                            log! {"success"};
                            set_show_modal(false);
                            set_current_modal(CurrentModal::Register);
                            set_user(Some(user));
                        }
                        Err(e) => {
                            log!("Error adding {:?}", e);
                            set_error_message(format! {"Error adding {:?}", e});
                        }
                    }
                });
            }
            Err(_) => {
                set_if_error(true);
                set_error_message(String::from("All fields are required!"));
            }
        }
    };

    let on_enter = move |e: ev::KeyboardEvent| {
        if e.key() == "Enter" {
            on_register(MouseEvent::new("").unwrap());
        }
    };

    log! {"creating component now!"};

    view! {
        <div class=style::container>
            <input type="email" placeholder="E-Mail"
                value=email
                on:input=move |e| {
                    set_email(event_target_value(&e));
                }
                class=style::input
            />
            <input type="password" placeholder="Passwort"
                value=password
                on:input=move |e| {
                    set_password(event_target_value(&e));
                }
                on:keydown = on_enter
                class=style::input
            />
            <span class=style::error_label>
                <Show when = move || { if_error() }>
                    {error_message()}
                </Show>
            </span>
            <button on:click=on_register class=style::button>"Registrieren"</button>
            <a class=style::link on:click=on_login_pressed>"Ich habe bereits ein Konto"</a>
        </div>
    }
}


#[server(SignUp, "/api")]
pub async fn sign_up(add_user_request: RegisterRequest) -> Result<User, ServerFnError> {
    use actix_web::{cookie::{time::Duration, Cookie, SameSite},  http::{header::{self, HeaderValue}, StatusCode}};
    use leptos_actix::ResponseOptions;
    let user = add_new_user(add_user_request.email, add_user_request.password.to_owned()).await;
    match user {
        Ok(res) => {
            let response = expect_context::<ResponseOptions>();
            let cookie = Cookie::build("auth_token", &res.1)
                    .http_only(true)
                    .secure(true)
                    .same_site(SameSite::Lax)
                    .path("/")
                    .max_age(Duration::days(30))
                    .finish();
            response.set_status(StatusCode::OK);
            if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
                response.insert_header(header::SET_COOKIE, cookie);
            }
            Ok(res.0)
        },
        Err(_) => Err(ServerFnError::Args(String::from("Error signing up!"))),
    }
}


#[server(Authenticate, "/api")]
pub async fn authenticate(
    authenticate_request: Option<AuthenticateRequest>,
) -> Result<User, ServerFnError> {
    use actix_web::HttpRequest;
    let http_request = use_context::<HttpRequest>();
    log!("HTTP: {:?}", http_request);
    let token = match authenticate_request {
        Some(auth_request) => auth_request.token,
        None => {
            let cookie = match http_request {
                Some(request) => match request.cookie("auth_token") {
                    Some(c) => c,
                    None => return Err(ServerFnError::Args(String::from("No cookie provided")))
                },
                None => return Err(ServerFnError::Args(String::from("No valid request"))),
            };
            cookie.value().to_owned()
        }
    };
    let claims = match validate_jwt(&token).await {
        Ok(decoded) => decoded,
        Err(_) => return Err(ServerFnError::Args(String::from("Invalid Token")))
    };
    
    match get_user_by_id(claims.sub).await {
        Some(user) => Ok(user),
        None => Err(ServerFnError::Args(String::from("User not found"))),
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::app::database;
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

        async fn get_user_by_id(uuid: String) -> Option<User> {
            database::get_user_by_id(uuid).await
        }

        async fn generate_password_hash(password: String) -> Result<String, argon2::password_hash::Error> {
            let salt = SaltString::generate(&mut OsRng);

            // Argon2 with default params (Argon2id v19)
            let argon2 = Argon2::default();

            // Hash password to PHC string ($argon2id$v=19$...)
            let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

            // Verify password against PHC string.
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
            log!("UUID: {:?}\nToken: {:?}\n decoded: {:?}", user_id, token, validate_jwt(token.as_ref().unwrap().as_str()).await);
            token
        }

        async fn validate_jwt(token: &str) -> Result<JWTClaims, jsonwebtoken::errors::Error> {
            let secret = env::var("JWT_KEY").unwrap();
            let decoded = decode::<JWTClaims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())?;
            Ok(decoded.claims)
        }

        async fn add_new_user<T>(email: T, password: T)
            -> Result<(User, String), UserError> where T: Into<String> {

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

            let token = match database::add_user(new_user.clone()).await {
                Some(_user) => generate_jwt(uuid).await,
                None => return Err(UserError::UserCreationFailure),
            };

            match token {
                Ok(token) => Ok((new_user,token)),
                Err(_e) => Err(UserError::UserCreationFailure),
            }
        }
    }
}
