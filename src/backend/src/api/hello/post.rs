use actix_web::{post, HttpResponse, Responder};

#[post("/api/hello")]
pub async fn json_response() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"message\": \"Hello World\"}")
}
