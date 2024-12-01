use leptos::*;

stylance::import_style!(style, "events.module.scss");

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