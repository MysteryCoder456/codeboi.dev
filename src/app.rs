use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod home;
mod models;

use home::HomePage;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Title text="CodeBoi"/>

        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors/> }.into_view(cx)
        }>
            <div class="main-container">
                <main>
                    <Routes>
                        <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
