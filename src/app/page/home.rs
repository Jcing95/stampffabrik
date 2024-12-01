use leptos::*;
use crate::app::components::{Intro, Events};


#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="container">
            <Intro/>
            <Events/>
        </div>
    }
}