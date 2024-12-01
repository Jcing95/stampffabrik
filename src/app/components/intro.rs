use leptos::*;

stylance::import_style!(my_style, "intro.module.scss");

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class=format!("component {}", my_style::component)>
        <div class=my_style::logo_container>
            <img class=my_style::logo src="assets/stampffabrik_1024.png"/>
            <img class=my_style::logo_glow src="assets/stampffabrik_1024_glow.png"/>
        </div>
            <div class=my_style::arrow><a href="#events" class="bi bi-caret-down-fill"></a></div>
        </div>
    }
}