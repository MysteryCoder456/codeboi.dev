use cfg_if::cfg_if;
use leptos_axum::handle_server_fns_with_context;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[macro_use]
        extern crate dotenv_codegen;

        use axum::{
            body::Body,
            extract::{FromRef, State, Path, RawQuery},
            http::{Request, HeaderMap},
            response::{IntoResponse, Response},
            routing::{get, post},
            Router,
        };
        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes};
        use portfolio::app::*;
        use portfolio::fileserv::file_and_error_handler;
        use sqlx::postgres::{PgPool, PgPoolOptions};

        cfg_if! {
            if #[cfg(feature = "tls")] {
                use std::path::Path;
                use axum_server::tls_rustls::RustlsConfig;
            }
        }

        #[derive(FromRef, Debug, Clone)]
        struct AppState {
            leptos_options: LeptosOptions,
            pool: PgPool,
        }

        async fn server_fn_handler(
            Path(fn_name): Path<String>,
            headers: HeaderMap,
            RawQuery(raw_query): RawQuery,
            State(app_state): State<AppState>,
            req: Request<Body>,
        ) -> impl IntoResponse {
            let handler = handle_server_fns_with_context(
                Path(fn_name),
                headers,
                RawQuery(raw_query),
                move || {
                    provide_context(app_state.pool.clone());
                },
                req
            );
            handler.await.into_response()
        }

        async fn leptos_routes_handler(
            State(app_state): State<AppState>,
            req: Request<Body>,
        ) -> Response {
            let handler = leptos_axum::render_app_to_stream_with_context(
                app_state.leptos_options.clone(),
                move || {
                    provide_context(app_state.pool.clone());
                },
                || view! { <App/> },
            );
            handler(req).await.into_response()
        }

        #[tokio::main]
        async fn main() {
            simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

            // Setting get_configuration(None) means we'll be using cargo-leptos's env values
            // For deployment these variables are:
            // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
            // Alternately a file can be specified such as Some("Cargo.toml")
            // The file would need to be included with the executable when moved to deployment
            let conf = get_configuration(None).await.unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let routes = generate_route_list(|| view! { <App/> });

            // SQL connection pool
            let pool = PgPoolOptions::new()
                .max_connections(2)
                .connect(dotenv!("DATABASE_URL"))
                .await
                .unwrap();

            // Build application state
            let app_state = AppState {
                leptos_options: leptos_options.clone(),
                pool,
            };

            // build our application with a route
            let app = Router::new()
                .route("/api/*fn_name", post(server_fn_handler))
                .leptos_routes_with_handler(routes, get(leptos_routes_handler))
                .fallback(file_and_error_handler)
                .with_state(app_state);

            cfg_if! {
                if #[cfg(feature = "tls")] {
                    let config = RustlsConfig::from_pem_file(
                        Path::new("ssl/cert.pem"),
                        Path::new("ssl/cert.key"),
                    )
                    .await
                    .unwrap();

                    // run our app with axum_server's rustls server
                    log::info!("listening on https://{}", &addr);
                    axum_server::bind_rustls(addr, config)
                        .serve(app.into_make_service())
                        .await
                        .unwrap();
                } else {
                    // run our app with hyper
                    // `axum::Server` is a re-export of `hyper::Server`
                    log::info!("listening on http://{}", &addr);
                    axum::Server::bind(&addr)
                        .serve(app.into_make_service())
                        .await
                        .unwrap();
                }
            }

        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
