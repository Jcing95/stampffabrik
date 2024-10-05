cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {

        use crate::app::model::User;
        use crate::app::errors::{ UserError };
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{ Surreal};
        use once_cell::sync::Lazy;

        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() {
            let _ = DB.connect::<Ws>("127.0.0.1:8000").await;
            let _ = DB.signin(Root {
                username: "root",
                password: "root",
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
            let results = DB.create(("user", new_user.uuid.to_string()))
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

    }
}
