use std::env;

use actix_files::{Files, NamedFile};
use actix_rt::System;
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{middleware, web, App, HttpServer};

mod api;
mod args;
mod auth;
mod cors;
mod session;
mod ssr_routes;

use crate::api::hello::get::json_response_get;
use crate::api::hello::post::json_response;
use crate::api::protected::get::protected_endpoint;
use crate::api::space_x::get::json_get_space_x;
use crate::args::collect_args::collect_args;
use crate::auth::auth_middleware::Authentication;
use crate::cors::get_cors_options::get_cors_options;
use crate::session::flash_messages::set_up_flash_messages;
use crate::ssr_routes::login::login_form;
use crate::ssr_routes::post_login::post_login;

/// Register every route and the static frontend files.
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::get().to(login_form))
        .route("/login", web::post().to(post_login))
        .service(json_response)
        .service(json_response_get)
        .service(json_get_space_x)
        .service(protected_endpoint)
        .service(
            Files::new("/", "../frontend/dist/")
                .prefer_utf8(true)
                .index_file("index.html")
                .default_handler(fn_service(|req: ServiceRequest| async {
                    let (req, _) = req.into_parts();
                    let file = NamedFile::open_async("../frontend/dist/404.html").await?;
                    let res = file.into_response(&req);
                    Ok(ServiceResponse::new(req, res))
                })),
        );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = collect_args(env::args().collect());
    let host = args.host;
    let port = args.port.parse::<u16>().unwrap();
    let cors_url = args.cors_url;
    let cookie_domain = args.cookie_domain;

    // configure logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Set up the actix server
    let server = HttpServer::new(move || {
        let env = args.env.to_string();
        let cors = get_cors_options(env, cors_url.clone()); //Prod CORS URL address, for dev run the cors is set to *
        let auth_routes: Vec<String> = vec!["/auth/*".to_string()]; // Routes that require authentication

        // The services and wrappers are loaded from the last to first
        // Ensure all the wrappers are after routes and handlers
        App::new()
            .wrap(cors)
            .configure(configure_routes)
            .wrap(Authentication {
                routes: auth_routes,
            })
            .wrap(session::session_middleware::session_middleware(
                cookie_domain.clone(),
            ))
            .wrap(set_up_flash_messages())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim)) // Add this line to handle trailing slashes\
    })
    .bind((host, port))?;

    let server = server.run();

    System::current().arbiter().spawn(async {
        println!("Actix server has started 🚀");
    });

    server.await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    macro_rules! init_app {
        () => {
            test::init_service(
                App::new()
                    .configure(configure_routes)
                    .wrap(session::session_middleware::session_middleware(None))
                    .wrap(set_up_flash_messages()),
            )
            .await
        };
    }

    #[actix_rt::test]
    async fn test_hello_routes_are_registered() {
        let app = init_app!();

        let get = test::TestRequest::get().uri("/api/hello").to_request();
        assert!(test::call_service(&app, get).await.status().is_success());

        let post = test::TestRequest::post().uri("/api/hello").to_request();
        assert!(test::call_service(&app, post).await.status().is_success());
    }

    #[actix_rt::test]
    async fn test_login_form_route_is_registered() {
        let app = init_app!();

        let req = test::TestRequest::get().uri("/login").to_request();
        let body = test::call_and_read_body(&app, req).await;

        assert!(String::from_utf8(body.to_vec())
            .unwrap()
            .contains(r#"<form action="/login" method="post">"#));
    }

    #[actix_rt::test]
    async fn test_protected_route_rejects_anonymous_users() {
        let app = init_app!();

        let req = test::TestRequest::get().uri("/api/protected").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_unknown_route_falls_through_to_default_handler() {
        let app = init_app!();

        let req = test::TestRequest::get()
            .uri("/definitely-not-a-route")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}
