use leptos::prelude::*;

stylance::import_style!(style, "../../style/home.module.scss");

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="container">
            <Intro/>
            <Events/>
        </div>
    }
}

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class=format!("component {}", style::component)>
        <div class=style::logo_container>
            <img class=style::logo src="assets/stampffabrik_1024.png"/>
            <img class=style::logo_glow src="assets/stampffabrik_1024_glow.png"/>
        </div>
            <div class=style::arrow><a href="#events" class="bi bi-caret-down-fill"></a></div>
        </div>
    }
}

#[component]
pub fn Events() -> impl IntoView {
    view! {
        <div class="component" id="events">
            <div class="h2">Upcoming Events</div>
            <div class=style::event >
                <img src="assets/event_prisma.png"/>
            </div>
        </div>
    }
}