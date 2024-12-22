use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Router, Route, Routes},
    StaticSegment, WildcardSegment,
};

use crate::app::components::{Header, Footer};
use page::{HomePage, AccountPage};

pub mod components;
pub mod page;
pub mod auth;
pub mod database;
pub mod errors;
pub mod model;

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
