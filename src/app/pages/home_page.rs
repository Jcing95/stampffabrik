use crate::app::components::{Intro, Events};
use leptos;


#[leptos::component]
pub fn HomePage() -> impl leptos::IntoView {
    leptos::view! {
        <div class="container">
            <Intro/>
            <Events/>
        </div>
    }
}