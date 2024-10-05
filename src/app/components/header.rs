use leptos;
use super::auth::{RegisterForm};

stylance::import_style!(style, "header.module.scss");

#[leptos::component]
pub fn Header() -> impl leptos::IntoView {

    let (show_register_modal, set_show_register_modal) = leptos::create_signal(false);

    let (is_logged_in, set_logged_in) = leptos::create_signal(false);

    let account_clicked = move |_| {
        set_show_register_modal(!show_register_modal());
    };

    leptos::view! {
        <div class=style::hbar>
            <a href="/">
                <img src="/assets/stampffabrik_64.png" class=style::nav_icon/>Stampffabrik
            </a>
            <div class=style::nav_menu>
                <a class=style::menu_entry href="/"><i class="bi bi-house-door-fill"></i> Home</a>
                <a class=style::menu_entry on:click=account_clicked><i class="bi bi-person-circle"></i>
                 {move || if !is_logged_in() {" LOGIN"} else {" USERNAME"}}
                </a>
            </div>
        </div>
        <leptos::Show when = move || { show_register_modal() }>
            <RegisterForm set_show_register_modal/>
        </leptos::Show>
    }
}