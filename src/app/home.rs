use leptos::*;
use stylers::style;

use crate::app::models::Project;

#[server(GetPinnedProjects)]
async fn get_pinned_projects() -> Result<Vec<Project>, ServerFnError> {
    use sqlx::PgPool;
    let pool = use_context::<PgPool>().ok_or(ServerFnError::ServerError(
        "State `PgPool` not found.".to_owned(),
    ))?;

    sqlx::query_as!(Project, "SELECT * FROM projects WHERE pinned = true")
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
    };

    view! { class=style_class,
        <div align="center">
            <img
                src="/images/CB.png"
                alt="CodeBois's Profile Picture"
                width="200px"
                class="codeboi-pfp"
            />
            <h1>"codeboi"</h1>
            <h3 class="muted">"aka rehatbir singh irl ;)"</h3>
        </div>

        <br/>

        <div>
            <h2>"TL;DR"</h2>
            <div class="content">
                <p>"I'm a high school senior who"</p>
                <ul>
                    <li>"likes programming & tech 💻"</li>
                    <li>"plays the guitar 🎸"</li>
                    <li>"likes playing Table Tennis & Badminton 🏓"</li>
                    <li>"and (most importantly) is obsessed with cats 🐈"</li>
                </ul>
            </div>
        </div>

        <br/>

        <div>
            <h2>"Sneak Peak Projects"</h2>

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
                                                view! {
                                                    <br/>
                                                    <PinnedProject project/>
                                                    <br/>
                                                }
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
    }
}

#[component]
fn PinnedProject<'a>(project: &'a Project) -> impl IntoView {
    let style_class = style! {
        .pinned-project {
            box-shadow: 0px 0px 68px -22px var(--malachite);
            border: 1px solid var(--dim-gray);
        }
    };

    view! { class=style_class,
        <div class="pinned-project content">
            <h3>{&project.name}</h3>
            <p>{&project.description}</p>
        </div>
    }
}
