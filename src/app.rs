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
        <Title text="CodeBoi"/>

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
            <img src="/images/CB.png" alt="CodeBois's Profile Picture" width="200px" class="codeboi-pfp" />
            <h1>"codeboi"</h1>
        </div>

        <div>
            <h2>"TL;DR"</h2>
            <p>"I'm a high school senior who"</p>
            <ul>
                <li>"likes programming & tech 💻"</li>
                <li>"plays the guitar 🎸"</li>
                <li>"likes playing table tennis & badminton 🏓"</li>
                <li>"and (most importantly) is obsessed with cats 🐈"</li>
            </ul>
        </div>
    }
}
