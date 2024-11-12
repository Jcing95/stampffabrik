use leptos::{self, create_signal, provide_context, WriteSignal};
use leptos_meta::*;
use leptos_router;

use crate::app::components::{Header, Footer};
use page::{HomePage, AccountPage};
use model::User;
use leptos::logging::log;

pub mod components;
pub mod page;
pub mod server;
pub mod db;
pub mod errors;
pub mod model;

#[leptos::component]
pub fn App() -> impl leptos::IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (user, set_user) = create_signal::<Option<User>>(None);
    provide_context(user);
    leptos::spawn_local(authenticate(set_user));

    leptos::view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/stampffabrik.css"/>
        <Stylesheet href="https://cdnjs.cloudflare.com/ajax/libs/bootstrap-icons/1.8.1/font/bootstrap-icons.min.css"/>

        // sets the document title
        <Title text="STAMPFFABRIK"/>

        // content for this welcome page
        <leptos_router::Router>
            <Header/>
            <div>
                <leptos_router::Routes>
                    <leptos_router::Route path="/" view=HomePage/>
                    <leptos_router::Route path="/account" view=AccountPage/>
                    <leptos_router::Route path="/*any" view=NotFound/>
                </leptos_router::Routes>
            </div>
            <Footer/>
        </leptos_router::Router>
    }
}

async fn authenticate(set_user: WriteSignal<Option<User>>) {
    let user = match server::auth::authenticate(None).await {
        Ok(u) => u,
        Err(e) => {
            log!("Error: {:?}", e);
            return;
        }
    };
    set_user(Some(user));
} 

/// 404 - Not Found
#[leptos::component]
fn NotFound() -> impl leptos::IntoView {
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
        let resp = leptos::expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    leptos::view! {
        <h1>"Not Found"</h1>
    }
}
