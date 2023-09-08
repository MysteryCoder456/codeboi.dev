use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
        <Title text="Rehatbir's Portfolio"/>

        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
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

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div align="center">
            <h1>"codeboi"</h1>
            <h2><b>"TL;DR"</b></h2>
        </div>

        <ul>
            <li>"I'm a high school senior who"</li>
            <li>"likes programming & tech,"</li>
            <li>"plays the guitar and"</li>
            <li>"likes playing table tennis & badminton!"</li>
        </ul>
    }
}
