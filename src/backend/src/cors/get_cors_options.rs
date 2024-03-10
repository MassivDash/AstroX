use actix_cors::Cors;
use actix_web::http;

pub fn get_cors_options(env: String) -> Cors {
    if env == "prod" {
        Cors::default()
            .allowed_origin("https://astrox.spaceout.pl/")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600)
    } else {
        Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600)
    }
}
