use leptos::{logging::log, prelude::*};

use crate::app::auth::AuthForm;

stylance::import_style!(style, "header.module.scss");

#[component]
pub fn Header() -> impl IntoView {
    let (show_modal, set_show_modal) = signal(false);

    let account_clicked = move |_| {
        set_show_modal(!show_modal());
    };

    view! {
        <div class=style::hbar>
            <a href="/">
                <img src="/assets/stampffabrik_64.png" class=style::nav_icon/>Stampffabrik
            </a>
            <div class=style::nav_menu>
                <a class=style::menu_entry href="/">
                    <i class="bi bi-house-door-fill"></i>
                </a>
                <a class=style::menu_entry on:click=account_clicked>
                    <i class="bi bi-person-circle"></i>
                </a>
            </div>
        </div>
        <AuthForm show_modal set_show_modal/>
    }
}
