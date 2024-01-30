use icondata as i;
use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use stylers::style;
use time::Date;

#[derive(Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub short_description: String,
    pub long_description: String,
    pub url: Option<String>,
    pub technologies: Option<String>,
    pub pinned: bool,
    pub date_created: Date,
}

fn tech_str_to_icon(tech_str: &str) -> Option<i::Icon> {
    match tech_str {
        "python" => Some(i::SiPython),
        "rust" => Some(i::SiRust),
        "flask" => Some(i::SiFlask),
        "leptos" => Some(i::SiLeptos),
        "flutter" => Some(i::SiFlutter),
        "dart" => Some(i::SiDart),
        "discord" => Some(i::SiDiscord),
        "postgres" => Some(i::SiPostgresql),
        "socketio" => Some(i::SiSocketdotio),
        _ => None,
    }
}

#[server(GetProjects)]
pub async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    use sqlx::PgPool;
    let pool = use_context::<PgPool>().ok_or(ServerFnError::new("State `PgPool` not found."))?;

    sqlx::query_as!(Project, "SELECT * FROM projects ORDER BY date_created DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e))
}

#[server(GetPinnedProjects)]
pub async fn get_pinned_projects() -> Result<Vec<Project>, ServerFnError> {
    use sqlx::PgPool;
    let pool = use_context::<PgPool>().ok_or(ServerFnError::new("State `PgPool` not found."))?;

    sqlx::query_as!(
        Project,
        "SELECT * FROM projects WHERE pinned = true ORDER BY date_created DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e))
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = create_resource(|| (), |_| get_projects());

    view! {
        <Title text="CodeBoi's Projects"/>

        <a href="/" class="muted">
            "< Back"
        </a>

        <h2 align="center">Projects</h2>
        <div class="projects">
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=move |_| {
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
                                                    view! { <ProjectCard project=&project/> }
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
    }
}

#[component]
pub fn PinnedProject<'a>(project: &'a Project) -> impl IntoView {
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
            width: 40%;
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
            <img src=format!("/images/projects/{}.png", project.id)/>

            <div class="info">
                <h2>
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

                </h2>
                <p>{&project.short_description}</p>
            </div>
        </div>
    }
}

#[component]
pub fn ProjectCard<'a>(project: &'a Project) -> impl IntoView {
    // TODO: Add project image

    let style_class = style! {
        .project-card {
            margin: 22px 0px;
        }

        .tech-icon {
            margin-right: 6px;
        }
    };

    view! { class=style_class,
        <div class="project-card content" id=project.id>
            <h2>
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

            </h2>

            <p>{&project.long_description}</p>

            // TODO: tooltips

            {if let Some(ref technologies) = project.technologies {
                let tech_icons = technologies
                    .split(",")
                    .filter_map(tech_str_to_icon)
                    .map(|icon| {
                        view! { class=style_class,
                            <span class="tech-icon">
                                <Icon icon width="26px" height="26px"/>
                            </span>
                        }
                    })
                    .collect_view();
                view! {
                    <br/>
                    <h3>Tech Stack</h3>
                    <p>{tech_icons}</p>
                }
                    .into_view()
            } else {
                view! {}.into_view()
            }}

        </div>
    }
}
