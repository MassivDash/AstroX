use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_files::{Files, NamedFile};
use actix_web::dev::{ServiceRequest, ServiceResponse, fn_service};
use std::env;



#[post("/api/hello")]
async fn json_response() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"message\": \"Hello World\"}")
}

#[get("/api/hello")]
async fn json_response_get() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"message\": \"Hello World\"}")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let args: Vec<String> = env::args().collect();

    let mut host = "127.0.0.1";
    let mut port = 8080;    

    for arg in &args {
        if arg.starts_with("--host=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                host = split[1];
            }
        }
        if arg.starts_with("--port=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                port = split[1].parse::<u16>().unwrap();
            }
        }
    }


    HttpServer::new(|| {
        App::new()
        .service(json_response)
        .service(json_response_get)
        // .service(Files::new("/styles", "./frontend/dist/styles/").show_files_listing())
        // .service(Files::new("/images", "./frontend/dist/images/").show_files_listing())
        // .service(Files::new("/scripts", "./frontend/dist/scripts/").show_files_listing())
        .service(Files::new("/", "../frontend/dist/").prefer_utf8(true).index_file("index.html").default_handler(fn_service(|req: ServiceRequest| async {
            let (req, _) = req.into_parts();
            let file = NamedFile::open_async("../frontend/dist/404.html").await?;
            let res = file.into_response(&req);
            Ok(ServiceResponse::new(req, res))
        })))
    })
    .bind((host, port))?
    .run()
    .await
}
