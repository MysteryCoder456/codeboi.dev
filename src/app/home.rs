use leptos::*;

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

    view! { cx,
        <div align="center">
            <img src="/images/CB.png" alt="CodeBois's Profile Picture" width="200px" class="codeboi-pfp" />
            <h1>"codeboi"</h1>
        </div>

        <br />

        <div>
            <h2>"TL;DR"</h2>
            <p>"I'm a high school senior who"</p>
            <ul>
                <li>"likes programming & tech 💻"</li>
                <li>"plays the guitar 🎸"</li>
                <li>"likes playing Table Tennis & Badminton 🏓"</li>
                <li>"and (most importantly) is obsessed with cats 🐈"</li>
            </ul>
        </div>

        <br />

        <div>
            <h2>"Sneak Peak"</h2>

            <Transition
                fallback=move || view! { cx, <p>"Loading..."</p> }
            >
                <ErrorBoundary fallback=|cx, _| view! { cx,
                    <p>"oops"</p>
                }>
                    {
                        move || projects.read(cx)
                        .map(|projects| {
                            match projects {
                                Ok(projects) => {
                                    projects.iter().map(|proj| {
                                        view! { cx, <p>{&proj.name}</p> }
                                    })
                                    .collect_view(cx)
                                },
                                Err(e) => view! { cx, <p>{e.to_string()}</p> }.into_view(cx)
                            }
                        })
                    }
                </ErrorBoundary>
            </Transition>
        </div>
    }
}
