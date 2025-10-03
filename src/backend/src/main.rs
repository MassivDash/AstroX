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
use crate::api::space_x::get::json_get_space_x;
use crate::args::collect_args::collect_args;
use crate::auth::auth_middleware::Authentication;
use crate::cors::get_cors_options::get_cors_options;
use crate::session::flash_messages::set_up_flash_messages;
use crate::ssr_routes::login::login_form;
use crate::ssr_routes::post_login::post_login;

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
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(post_login))
            .service(json_response)
            .service(json_response_get)
            .service(json_get_space_x)
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
            )
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
