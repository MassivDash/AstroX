use actix_web::{post, HttpResponse, Responder};
use actix_web::{test, App};

#[post("/api/hello")]
pub async fn json_response() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"message\": \"Hello World\"}")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[actix_rt::test]
    async fn test_json_response() {
        let mut app = test::init_service(App::new().service(json_response)).await;

        let req = test::TestRequest::post().uri("/api/hello").to_request();

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "{\"message\": \"Hello World\"}");
    }
}
