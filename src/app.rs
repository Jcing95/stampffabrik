use leptos::{logging::log, prelude::*};
use leptos_meta::*;
use leptos_router::{
    components::{Router, Route, Routes},
    StaticSegment, WildcardSegment,
};

use auth::AuthForm;
use model::User;
use page::{HomePage, AccountPage};

pub mod page;
pub mod auth;
pub mod database;
pub mod errors;
pub mod model;

stylance::import_style!(style, "style/app.module.scss");

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/stampffabrik.css"/>
        <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/bootstrap-icons/1.8.1/font/bootstrap-icons.min.css"/>

        // sets the document title
        <Title text="STAMPFFABRIK"/>

        // content for this welcome page
        <Router>
            <main>
                <Header/>
                    <Routes fallback=move || "not found.">
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("account") view=AccountPage/>
                        <Route path=WildcardSegment("any") view=NotFound/>
                    </Routes>
                <Footer/>
            </main>
        </Router>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let (show_modal, set_show_modal) = signal(false);
    
    let get_user= || -> Option<ReadSignal<Option<User>>> {
        match use_context::<(ReadSignal<Option<User>>, WriteSignal<Option<User>>)>() {
            Some(t) => Some(t.0),
            None => None,
        }
    };
    let render_account = move || {
        view!{
            <Show when=move || get_user().is_none() || get_user().unwrap()().is_none()> 
                <a class=style::menu_entry on:click=move |_| set_show_modal(!show_modal())>
                    <i class="bi bi-person-circle"></i>
                </a>
            </Show>
            <Show when=move || get_user().is_some() && get_user().unwrap()().is_some()>
                <a class=style::menu_entry href="/account" on:click=move |_| ()>
                    <i class="bi bi-person-circle"></i>
                </a>
            </Show>
        }
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
                {render_account()}
            </div>
        </div>
        <AuthForm show_modal set_show_modal/>
    }
}

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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
