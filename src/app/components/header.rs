use crate::app::model::User;
use leptos::{self, expect_context, logging::log, ReadSignal};

use super::auth::AuthForm;

stylance::import_style!(style, "header.module.scss");

#[leptos::component]
pub fn Header() -> impl leptos::IntoView {
    let (show_modal, set_show_modal) = leptos::create_signal(false);

    let account_clicked = move |_| {
        set_show_modal(!show_modal());
    };

    let user = expect_context::<ReadSignal<Option<User>>>();
    log!("user from signal: {:?}", user);

    leptos::view! {
        <div class=style::hbar>
            <a href="/">
                <img src="/assets/stampffabrik_64.png" class=style::nav_icon/>Stampffabrik
            </a>
            <div class=style::nav_menu>
                <a class=style::menu_entry href="/"><i class="bi bi-house-door-fill"></i> Home</a>
                <a class=style::menu_entry on:click=account_clicked><i class="bi bi-person-circle"></i>
                { move ||
                    match user()  {
                        Some(_) => "Profile".to_string(),
                        None => "Login".to_string(),
                    }
                }
                </a>
            </div>
        </div>
        <AuthForm show_modal set_show_modal/>
    }
}
