use leptos::*;
use leptos_meta::*;
use stylers::style;

#[component]
pub fn BoxCollision() -> impl IntoView {
    let style_class = style! {
        #pi-canvas {
            margin-top: 8px;
        }
    };

    view! { class=style_class,
        <Title text="Box Collision"/>
        <script src="/wasm/pi_estimator/load.js" type="module" defer></script>

        <a href="/" class="muted">
            "< Back"
        </a>
        <br/>

        <h2 align="center">Box Collisions</h2>
        <div class="content" align="center">
            <p>
                A simple simulation where two boxes collide. The number of
                collisions has an interesting property: they resemble the
                digits of π. Your system may lag a bit, since
                the simulation runs at ~20,000 ticks per second for accurate
                collision detection.
            </p>
        </div>

        <canvas id="pi-canvas"></canvas>
    }
}
