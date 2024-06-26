use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::app_state::AppState;
use crate::server_functions::foo_bar;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-state-test.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let once = create_resource(
        || String::from("greg"),
        |_| {
            foo_bar(String::from("thing"))
        }
    );

    view! {
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
        {move || {
            match once.get() {
                Some(s) => match s {
                    Ok(x) => view! { <p>{x.that}</p> }.into_view(),
                    Err(e) => view! { <p>{ e.to_string() }</p> }.into_view()
                }
                None => view! { <p>"loading ..."</p> }.into_view(),
            }
        }}
        </Suspense>
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
