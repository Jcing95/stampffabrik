cfg_if::cfg_if! {

    if #[cfg(feature = "ssr")] {

        use crate::app::model::User;
        use crate::app::errors::{ ErrorMessage, UserError };
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() {

            let _ = DB.connect::<Ws>("127.0.0.1:8000").await;
            let _ = DB.signin(Root {
                username: "Root",
                password: "Root",
            })
            .await;
            let _ = DB.use_ns("surreal").use_db("user").await;
        }

        pub async fn get_all_users() -> Option<Vec<User>> {

            open_db_connection().await;
            let get_all_users = DB.query("SELECT * FROM user").await;
            let _ = DB.invalidate().await;

            match get_all_users {
                Ok(mut res) => {
                    let found = res.take(0);
                    match found {
                        Ok(found_users) => Some(found_users),
                        Err(_) => None,
                    }
                },
                Err(_) => None,
            }
        }

        pub async fn get_user_by_mail(email: String) -> Option<User> {
            open_db_connection().await;
            let user = DB.query("SELECT * FROM user WHERE email = $email").bind(("email", email)).await;
            let _ = DB.invalidate().await;

            match user {
                Ok(mut res) => {
                    let found:Result<Vec<User>,_> = res.take(0);
                    match found {
                        Ok(found_user) => Some(found_user[0].clone()),
                        Err(_) => None,
                    }
                },
                Err(_) => None,
            }
        }

        pub async fn add_user(new_user: User) -> Option<User> {
            open_db_connection().await;
            let results = DB.create(("user", new_user.uuid.clone()))
                .content(new_user)
                .await;
            let _ = DB.invalidate().await;

            match results {
                Ok(created_user) => created_user,
                Err(e) => {
                    println!("error in adding user: {:?}",e);
                    None
                }
            }
        }

        pub async fn delete_user(user_uuid: String)
            -> Result<Option<User>, UserError> {

            open_db_connection().await;
            let delete_results = DB.delete(("user",user_uuid)).await;
            let _ = DB.invalidate().await;

            match delete_results {
                Ok(deleted_user) => Ok(deleted_user),
                Err(_) => Err(UserError::UserDeleteFailure)
            }
        }

        pub async fn update_user(uuid: String, email: String, password_hash: String,
            ) -> Result<Option<User>,UserError> {

            open_db_connection().await;

            // first we try to find the perosn in the database
            let find_user: Result<Option<User>,Error> =
                DB.select(("user",&uuid)).await;
            match find_user {

                Ok(found) => {

                    // if we found the user, we update him/her
                    match found {
                        Some(found_user) => {

                            let updated_user: Result<Option<User>,Error> =
                                DB.update(("user",&uuid))
                                .merge(User::new(
                                    uuid,
                                    email,
                                    password_hash,
                                    found_user.joined_date,
                                ))
                                .await;
                            let _ = DB.invalidate().await;
                            match updated_user {
                                Ok(returned_user) => Ok(returned_user),
                                Err(_) => Err(UserError::UserUpdateFailure)
                            }
                        },
                        None => Err(UserError::UserUpdateFailure)
                    }
                },
                Err(_) => {
                    let _ = DB.invalidate().await;
                    Err(UserError::UserNotFound)
                }
            }
        }
    }
}
