use leptos::{logging::log, prelude::*};

use crate::app::model::User;


#[leptos::component]
pub fn AccountPage() -> impl IntoView {
    let (get_user, set_user) = expect_context::<(ReadSignal<Option<User>>, WriteSignal<Option<User>>)>();
    let user = get_user().unwrap();
    view! {
        <div>
        {user.name}
        {user.email}
        </div>
        // <h2>{user.name}</h2>
        // <h3>{user.email}</h3>
        <h3>Address</h3>
    }
}