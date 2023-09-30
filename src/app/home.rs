use leptos::*;
use stylers::style;

use crate::app::models::Project;

#[server(GetPinnedProjects)]
async fn get_pinned_projects(cx: Scope) -> Result<Vec<Project>, ServerFnError> {
    use sqlx::PgPool;
    let pool = use_context::<PgPool>(cx).ok_or(ServerFnError::ServerError(
        "State `PgPool` not found.".to_owned(),
    ))?;

    sqlx::query_as!(Project, "SELECT * FROM projects WHERE pinned = true")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let projects = create_resource(cx, || (), move |_| get_pinned_projects(cx));

    let style_class = style! {
        .codeboi-pfp {
            box-shadow: 0px 0px 42px -12px var(--malachite);
        }
    };

    view! { cx, class=style_class,
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

            <Transition fallback=move || view! { cx, <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|cx, _| {
                    view! { cx, <p>"oops"</p> }
                }>
                    {move || {
                        projects
                            .read(cx)
                            .map(|projects| {
                                match projects {
                                    Ok(projects) => {
                                        projects
                                            .iter()
                                            .map(|project| {
                                                view! { cx,
                                                    <br/>
                                                    <PinnedProject project/>
                                                    <br/>
                                                }
                                            })
                                            .collect_view(cx)
                                    }
                                    Err(e) => view! { cx, <p>{e.to_string()}</p> }.into_view(cx),
                                }
                            })
                    }}
                </ErrorBoundary>
            </Transition>
        </div>
    }
}

#[component]
fn PinnedProject<'a>(cx: Scope, project: &'a Project) -> impl IntoView {
    let style_class = style! {
        .pinned-project {
            box-shadow: 0px 0px 42px -12px var(--malachite);
        }
    };

    view! { cx, class=style_class,
        <div class="pinned-project content">
            <h3>{&project.name}</h3>
            <p>{&project.description}</p>
        </div>
    }
}
