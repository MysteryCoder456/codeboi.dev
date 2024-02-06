use icondata as i;
use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use stylers::style;

use crate::app::projects::{get_pinned_projects, PinnedProjectCard};

#[component]
pub fn HomePage() -> impl IntoView {
    let projects = create_resource(|| (), move |_| get_pinned_projects());

    let style_class = style! {
        .codeboi-pfp {
            outline: 4px solid rgba(255, 255, 255, 0.3);
            outline-offset: 4px;
            background-clip: padding-box;
        }

        .socials > a {
            margin: 0px 12px;
            transition: 0.12s opacity;
        }

        .socials > a:hover {
            opacity: 1.0;
        }

        .transparent {
            color: var(--ghost-white);
            opacity: 0.45;
        }

        .pinned-projects {
            width: 100%;
            display: flex;
            flex-direction: column;
            gap: 16px;
        }
    };

    view! { class=style_class,
        <Title text="CodeBoi"/>

        <div align="center">
            <img
                src="/images/CB.png"
                alt="CodeBoi's Profile Picture"
                width="200px"
                class="codeboi-pfp"
            />
            <h1>"CodeBoi"</h1>
            <h3 class="transparent">"aka rehatbir singh irl ;)"</h3>
        </div>

        <div class="socials" align="center">
            <a href="https://github.com/mysterycoder456" title="GitHub" target="_blank" class="transparent">
                <Icon icon=i::BsGithub width="40px" height="40px"/>
            </a>

            <a href="https://monkeytype.com/profile/CodeBoi" title="MonkeyType" target="_blank" class="transparent">
                <Icon icon=i::BsKeyboardFill width="40px" height="40px"/>
            </a>

            <a
                href="https://www.linkedin.com/in/rehatbir-singh-4805ba193"
                title="LinkedIn"
                target="_blank" class="transparent"
            >
                <Icon icon=i::BsLinkedin width="40px" height="40px"/>
            </a>

            <a href="https://www.fiverr.com/rehatbirsingh" title="Fiverr" target="_blank" class="transparent">
                <Icon icon=i::SiFiverr width="40px" height="40px"/>
            </a>

            <a href="mailto:rehatbir.singh@gmail.com" title="Mail" target="_blank" class="transparent">
                <Icon icon=i::TbMailFilled width="40px" height="40px"/>
            </a>
        </div>

        <br/>

        <h2>"TL;DR"</h2>
        <div class="content content-border" align="center">
            <p>"I'm a high school senior who"</p>
            <div align="left">
                <ul>
                    <li>"likes programming & tech 💻"</li>
                    <li>"plays the guitar 🎸"</li>
                    <li>"likes playing Table Tennis & Badminton 🏓"</li>
                    <li>"and (most importantly) is obsessed with cats 🐈"</li>
                </ul>
            </div>
        </div>

        <br/>

        <h2>"Fun Stuff"</h2>
        <div class="content content-border">
            <ul>
                <li>
                    <a href="/box">Box Collisions (Digits of Pi)</a>
                </li>
            </ul>
        </div>

        <br/>

        <h2>"Project Sneak Peaks"</h2>
        <div class="pinned-projects">
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|_| {
                    view! { <p>"oops"</p> }
                }>
                    {move || {
                        projects
                            .get()
                            .map(|projects| {
                                match projects {
                                    Ok(projects) => {
                                        view! {
                                            <For
                                                each=move || projects.clone()
                                                key=|p| p.id
                                                children=move |project| {
                                                    view! { <PinnedProjectCard project=&project/> }
                                                }
                                            />
                                        }
                                    }
                                    Err(e) => view! { <p>{e.to_string()}</p> }.into_view(),
                                }
                            })
                    }}

                </ErrorBoundary>
            </Transition>
        </div>
        <div align="center">
            <a href="/projects">
                <h3>View More</h3>
            </a>
        </div>
    }
}
