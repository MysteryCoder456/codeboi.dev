use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use stylers::style;
use time::Date;

#[derive(Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url: Option<String>,
    pub pinned: bool,
    pub date_created: Date,
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
                                                children=move |project| view! { <p>{project.name}</p> }
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
