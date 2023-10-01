use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod home;
mod models;

use home::HomePage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Stylesheet href="/stylers.css"/>
        <Title text="CodeBoi"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <div class="main-container">
                <main>
                    <Routes>
                        <Route path="" view=|| view! { <HomePage/> }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
