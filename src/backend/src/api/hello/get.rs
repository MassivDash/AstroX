use actix_web::{get, HttpResponse, Responder};

#[get("/api/hello")]
async fn json_response_get() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"message\": \"Hello World\"}")
}
