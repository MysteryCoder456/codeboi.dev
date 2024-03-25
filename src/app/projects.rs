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
        "opengl" => Some(i::SiOpengl),
        "javascript" => Some(i::SiJavascript),
        "rocket" => Some(i::BsRocketTakeoffFill),
        "firebase" => Some(i::SiFirebase),
        "swift" => Some(i::SiSwift),
        "redis" => Some(i::SiRedis),
        _ => None,
    }
}

fn capitalize_str(input: &str) -> String {
    input
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            if i == 0 {
                ch.to_ascii_uppercase()
            } else {
                ch.to_ascii_lowercase()
            }
        })
        .collect()
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
pub fn PinnedProjectCard<'a>(project: &'a Project) -> impl IntoView {
    let style_class = style! {
        .pinned-project {
            align-items: center;
            margin: 5px 0px;
            display: grid;
            grid-template-columns: 50% 50%;
            gap: 10px;
        }

        .pinned-project img {
            max-width: 100%;
            height: auto;
            box-shadow: 0px 0px 8px -1px black;
            border-radius: 8px;
            margin: 8px auto;
        }

        .info {
            text-align: center;
        }

        @media (max-width: 600px) {
            .pinned-project {
                grid-template-columns: 1fr;
                grid-template-rows: 1fr max-content;
            }

            .pinned-project img {
                width: auto;
                height: 12rem;
            }
        }
    };

    view! { class=style_class,
        <div class="pinned-project content content-border">
            <img src=format!("/images/projects/{}.png", project.id)/>

            <div class="info">
                <h2>
                    // TODO: Change this to link to project card or detail
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
    let style_class = style! {
        .project-card {
            background-color: var(--gunmetal);
            margin: 32px 0px;
            display: grid;
            grid-template-rows: 1fr max-content;
        }

        .project-img {
            width: 100%;
            object-fit: cover;
            max-height: "25em";
            border-radius: 12px 12px 0px 0px;
        }

        .project-info {
            padding: 0px 25px 12px 25px;
        }

        .tech-stack {
            display: flex;
            gap: "0.5em";
        }

        .tech-icon {
            width: min-content;
            height: min-content;
        }
    };

    view! { class=style_class,
        <div class="project-card content-border" id=project.id>
            // TODO: Image blur effect
            <img src=format!("/images/projects/{}.png", project.id) class="project-img"/>

            <div class="project-info">
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

                {if let Some(ref technologies) = project.technologies {
                    let tech_icons = technologies
                        .split(",")
                        .filter_map(|tech_name| {
                            let trimmed_name = tech_name.trim();
                            tech_str_to_icon(trimmed_name)
                                .map(|icon| (capitalize_str(trimmed_name), icon))
                        })
                        .map(|(icon_name, icon)| {
                            view! { class=style_class,
                                <span class="tech-icon" title=icon_name>
                                    <Icon icon width="2em" height="auto"/>
                                </span>
                            }
                        })
                        .collect_view();
                    view! { class=style_class,
                        <br/>
                        <h3>Tech Stack</h3>
                        <p class="tech-stack">{tech_icons}</p>
                    }
                        .into_view()
                } else {
                    view! {}.into_view()
                }}

            </div>
        </div>
    }
}
