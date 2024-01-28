use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[macro_use]
        extern crate dotenv_codegen;

        use axum::Router;
        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes};
        use portfolio::app::*;
        use portfolio::fileserv::file_and_error_handler;
        use sqlx::postgres::PgPoolOptions;

        cfg_if! {
            if #[cfg(feature = "tls")] {
                use std::path::Path as StdPath;
                use axum_server::tls_rustls::RustlsConfig;
            } else {
                use tokio::net::TcpListener;
            }
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
                .max_connections(4)
                .connect(dotenv!("DATABASE_URL"))
                .await
                .unwrap();

            // build our application with a route
            let app = Router::new()
                .leptos_routes_with_context(&leptos_options, routes, move || {
                    provide_context(pool.clone());
                }, App)
                .fallback(file_and_error_handler)
                .with_state(leptos_options);

            cfg_if! {
                if #[cfg(feature = "tls")] {
                    let config = RustlsConfig::from_pem_file(
                        StdPath::new("ssl/cert.pem"),
                        StdPath::new("ssl/cert.key"),
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
                    let listener = TcpListener::bind(addr).await.unwrap();
                    log::info!("listening on http://{}", &addr);
                    axum::serve(listener, app.into_make_service()).await.unwrap();
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
