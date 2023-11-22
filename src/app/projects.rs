use leptos::*;
use leptos_meta::*;

use crate::app::models::Project;

#[server(GetProjects)]
async fn get_projects() -> Result<Vec<Project>, ServerFnError> {
    use sqlx::PgPool;
    let pool = use_context::<PgPool>().ok_or(ServerFnError::ServerError(
        "State `PgPool` not found.".to_owned(),
    ))?;

    sqlx::query_as!(Project, "SELECT * FROM projects ORDER BY date_created DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="CodeBoi's Projects"/>

        <a href="/" class="muted">
            "< Back"
        </a>

        <h2>Projects</h2>
        <div class="content">
            <p>Coming Soon...</p>
        </div>
    }
}
