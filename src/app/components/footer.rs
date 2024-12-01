use leptos::*;

stylance::import_style!(style, "footer.module.scss");

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class=style::footer>
            <span class=style::social>
                <a class="bi bi-instagram" href="https://www.instagram.com/stampffabrik"></a>
                <a class="bi bi-facebook" href="https://www.facebook.com/stampffabrik"></a>
            </span>
            <a class=style::mail href="mailto:mail@stampffabrik.de">mail@stampffabrik.de</a>
            <span inner_html="&copy; 2024 Stampffabrik"></span>
        </div>
    }
}
