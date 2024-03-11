use actix_files::{Files, NamedFile};
use actix_rt::System;
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::{middleware, App, HttpServer};

mod api;
mod args;
mod cors;

use crate::args::collect_args::collect_args;
use crate::cors::get_cors_options::get_cors_options;

use crate::api::hello::get::json_response_get;
use crate::api::hello::post::json_response;
use crate::api::space_x::get::json_get_space_x;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = collect_args();

    let host = args.host;
    let port = args.port.parse::<u16>().unwrap();

    // configure logging
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let server = HttpServer::new(move || {
        let env = args.env.to_string();
        let cors = get_cors_options(env);
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(cors)
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
    })
    .bind((host, port))?;

    let server = server.run();

    System::current().arbiter().spawn(async {
        println!("----");
        println!("HttpServer has started");
        println!("----");
    });

    server.await
}
