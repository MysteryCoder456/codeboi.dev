use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod box_collision;
mod home;
mod projects;

use box_collision::BoxCollision;
use home::HomePage;
use projects::Projects;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let meta_description = "Hi there! I'm Rehatbir, and welcome to my humble abode on the internet :D I am a high school student who likes to code, play guitar, and loves learning.";

    view! {
        <Stylesheet id="leptos" href="/pkg/portfolio.css"/>

        <Meta name="og:title" content="CodeBoi"/>
        <Meta name="og:image" content="/images/CB.png"/>
        <Meta name="og:url" content="https://codeboi.dev"/>
        <Meta name="theme-color" content="#09e85e"/>
        <Meta name="og:description" content=meta_description/>
        <Meta name="description" content=meta_description/>

        <Link rel="preload" as_="font" href="/fonts/UbuntuMono-Regular.ttf"/>
        <Link rel="preload" as_="font" href="/fonts/UbuntuMono-Bold.ttf"/>
        <Link rel="preload" as_="font" href="/fonts/UbuntuMono-Italic.ttf"/>
        <Link rel="preload" as_="font" href="/fonts/UbuntuMono-BoldItalic.ttf"/>

        <Script
            async_="true"
            src="https://umami.codeboi.dev/script.js"
            attrs=vec![
                (
                    "data-website-id",
                    Attribute::String("6d05defe-708b-41d7-91c1-40a55ec94f3d".into()),
                ),
            ]
        />

        <svg class="bg-design">
            <radialGradient id="Gradient1">
                <stop offset="0%" stop-color="#00ff61"></stop>
                <stop offset="100%" stop-color="#00732c"></stop>
            </radialGradient>

          <ellipse cy="50%" data-hk="0-0-0-28" fill="url(#Gradient1)" cx="50%" rx="32rem" ry="16rem"></ellipse>
        </svg>

        <div class="main-container">
            <main>
                <Router fallback=|| {
                    let mut outside_errors = Errors::default();
                    outside_errors.insert_with_default_key(AppError::NotFound);
                    view! { <ErrorTemplate outside_errors/> }.into_view()
                }>
                    <Routes>
                        <Route path="" view=|| view! { <HomePage/> }/>
                        <Route path="/projects" view=|| view! { <Projects/> }/>
                        <Route path="/box" view=|| view! { <BoxCollision/> }/>
                    </Routes>
                </Router>
            </main>
        </div>

        <hr class="footer-hr"/>
        <footer>
            <p class="muted">Made by CodeBoi with Rust + Leptos + lots of love</p>
            <a href="https://github.com/MysteryCoder456/codeboi.dev" class="muted" target="_blank">
                View Source on GitHub
            </a>
        </footer>
    }
}
