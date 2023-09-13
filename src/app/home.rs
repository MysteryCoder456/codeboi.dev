use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
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
            // TODO: some proud projects and stuff
        </div>
    }
}
