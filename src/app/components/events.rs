use leptos;

stylance::import_style!(style, "events.module.scss");

#[leptos::component]
pub fn Events() -> impl leptos::IntoView {
    leptos::view! {
        <div class="component" id="events">
            <div class="h2">Upcoming Events</div>
            <div class=style::event >
                <img src="assets/event_prisma.png"/>
            </div>
        </div>
    }
}