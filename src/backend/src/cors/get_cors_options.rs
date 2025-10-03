use actix_cors::Cors;
use actix_web::http;

/// Gets the CORS options based on the environment and allowed origin.
///
/// # Arguments
///
/// * `env` - A `String` representing the environment.
/// * `allowed_origin` - A `String` representing the allowed origin.
///
/// # Returns
///
/// The `Cors` options based on the environment and allowed origin.
///
/// # Examples
///
/// ```
/// let env = String::from("prod");
/// let allowed_origin = String::from("https://astrox.spaceout.pl");
/// let cors = get_cors_options(env, allowed_origin);
/// ```
pub fn get_cors_options(env: String, allowed_origin: String) -> Cors {
    if env == "prod" {
        Cors::default()
            .allowed_origin(&allowed_origin)
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, web, App, HttpResponse, Responder};

    //manual route for testing
    async fn manual_hello() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }

    #[actix_web::test]
    async fn test_index_prod_get() {
        let env = String::from("prod");
        let cors = get_cors_options(env, String::from("https://astrox.spaceout.pl"));

        let app = test::init_service(
            App::new()
                .wrap(cors)
                .route("/", web::get().to(manual_hello)),
        )
        .await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_dev_get() {
        let env = String::from("dev");
        let cors = get_cors_options(env, String::from("https://astrox.spaceout.pl"));

        let app = test::init_service(
            App::new()
                .wrap(cors)
                .route("/", web::get().to(manual_hello)),
        )
        .await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_success());
    }
}
