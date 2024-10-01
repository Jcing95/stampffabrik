use leptos;

stylance::import_style!(style, "header.module.scss");

#[leptos::component]
pub fn Header() -> impl leptos::IntoView {
    leptos::view! {
        <div class=style::hbar>
            <a href="/">
                <img src="/assets/stampffabrik_64.png" class=style::nav_icon/>Stampffabrik
            </a>
            <div class=style::nav_menu>
                <a class=style::menu_entry href="/"><i class="bi bi-house-door-fill"></i> Home</a>
                <a class=style::menu_entry href="/account"><i class="bi bi-person-circle"></i> Account</a>
            </div>
        </div>
    }
}