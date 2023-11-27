use leptos::*;
use leptos_icons::{BsIcon::*, SiIcon::*, TbIcon::*, *};
use leptos_meta::*;
use stylers::style;

use crate::app::models::Project;

#[server(GetPinnedProjects)]
async fn get_pinned_projects() -> Result<Vec<Project>, ServerFnError> {
    use sqlx::PgPool;
    let pool = use_context::<PgPool>().ok_or(ServerFnError::ServerError(
        "State `PgPool` not found.".to_owned(),
    ))?;

    sqlx::query_as!(
        Project,
        "SELECT * FROM projects WHERE pinned = true ORDER BY date_created DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn HomePage() -> impl IntoView {
    let projects = create_resource(|| (), move |_| get_pinned_projects());

    let style_class = style! {
        .codeboi-pfp {
            box-shadow: 0px 0px 42px -12px var(--malachite);
        }

        .socials > a {
            color: var(--dim-gray);
            margin: 0px 12px;
            transition: 0.12s color;
        }

        .socials > a:hover {
            color: var(--ghost-white);
        }

        .pinned-projects {
            width: 100%;
            display: flex;
            flex-direction: column;
            gap: 40px;
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
            <h3 class="muted">"aka rehatbir singh irl ;)"</h3>
        </div>

        <div class="socials" align="center">
            <a href="https://github.com/mysterycoder456" title="GitHub" target="_blank">
                <Icon icon=Icon::from(BsGithub) width="40px" height="40px"/>
            </a>

            <a href="https://monkeytype.com/profile/CodeBoi" title="MonkeyType" target="_blank">
                <Icon icon=Icon::from(BsKeyboardFill) width="40px" height="40px"/>
            </a>

            <a
                href="https://www.linkedin.com/in/rehatbir-singh-4805ba193"
                title="LinkedIn"
                target="_blank"
            >
                <Icon icon=Icon::from(BsLinkedin) width="40px" height="40px"/>
            </a>

            <a href="https://www.fiverr.com/rehatbirsingh" title="Fiverr" target="_blank">
                <Icon icon=Icon::from(SiFiverr) width="40px" height="40px"/>
            </a>

            <a href="mailto:rehatbir.singh@gmail.com" title="Mail" target="_blank">
                <Icon icon=Icon::from(TbMailFilled) width="40px" height="40px"/>
            </a>
        </div>

        <br/>

        <h2>"TL;DR"</h2>
        <div class="content" align="center">
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
        <div class="content">
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
                                        projects
                                            .iter()
                                            .map(|project| {
                                                view! { <PinnedProject project/> }
                                            })
                                            .collect_view()
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

#[component]
fn PinnedProject<'a>(project: &'a Project) -> impl IntoView {
    let style_class = style! {
        .pinned-project {
            display: flex;
            flex-direction: row;
            justify-content: space-around;
            align-items: center;
            box-shadow: 0px 0px 64px -24px var(--malachite);
            margin: 5px 0px;
        }

        .pinned-project img {
            width: 50%;
            height: auto;
            box-shadow: 0px 0px 8px -1px black;
            border-radius: 8px;
            margin: 8px 0px;
        }

        @media (max-width: 600px) {
            .pinned-project {
                flex-direction: column;
            }

            .pinned-project img {
                width: auto;
                height: 12rem;
            }
        }
    };

    view! { class=style_class,
        <div class="pinned-project content">
            <div class="info">
                <h3>
                    {if let Some(ref url) = project.url {
                        view! {
                            <a href=url target="_blank">
                                {&project.name}
                            </a>
                        }
                            .into_view()
                    } else {
                        view! { <span>{&project.name}</span> }.into_view()
                    }}

                </h3>
                <p>{&project.description}</p>
            </div>

            <img src=format!("/images/projects/{}.png", project.id)/>
        </div>
    }
}
