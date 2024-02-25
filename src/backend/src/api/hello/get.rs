use actix_web::{get, HttpResponse, Responder};

#[get("/api/hello")]
async fn json_response_get() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"message\": \"Hello World\"}")
}
#[cfg(test)]
use actix_web::{test, App};
mod tests {
    #[actix_rt::test]
    async fn test_json_response_get() {
        let mut app = test::init_service(App::new().service(json_response_get)).await;

        let req = test::TestRequest::get().uri("/api/hello").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "{\"message\": \"Hello World\"}");
    }
}
