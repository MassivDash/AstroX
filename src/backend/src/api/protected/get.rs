use crate::session::validate_session::validate_session;
use actix_session::Session;
use actix_web::{get, HttpResponse, Responder};

#[get("/api/protected")]
pub async fn protected_endpoint(session: Session) -> impl Responder {
    // Validate that the user has a valid session
    match validate_session(&session) {
        Ok(_) => {
            // User is authenticated, return protected data
            HttpResponse::Ok().content_type("application/json").body(
                r#"{"message": "Welcome! This is protected data.", "status": "authenticated"}"#,
            )
        }
        Err(_) => {
            // User is not authenticated, return 401 Unauthorized
            HttpResponse::Unauthorized()
                .content_type("application/json")
                .body(r#"{"error": "Unauthorized", "message": "You must be logged in to access this resource"}"#)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::flash_messages::set_up_flash_messages;
    use crate::session::session_middleware::session_middleware;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_protected_endpoint_unauthorized() {
        // Test without a valid session - should return 401
        let mut app = test::init_service(
            App::new()
                .service(protected_endpoint)
                .wrap(set_up_flash_messages())
                .wrap(session_middleware()),
        )
        .await;

        let req = test::TestRequest::get().uri("/api/protected").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status().as_u16(), 401);

        let body = test::read_body(resp).await;
        assert!(body.to_vec().len() > 0);
    }

    #[actix_rt::test]
    async fn test_protected_endpoint_authorized() {
        // Test with a valid session - should return 200
        use actix_web::web;

        // Helper function to set up a session
        async fn setup_session(session: Session) -> impl Responder {
            let _ = session.insert("user_id", 42);
            HttpResponse::Ok().body("Session set")
        }

        let mut app = test::init_service(
            App::new()
                .route("/setup", web::get().to(setup_session))
                .service(protected_endpoint)
                .wrap(set_up_flash_messages())
                .wrap(session_middleware()),
        )
        .await;

        // First, set up the session
        let req = test::TestRequest::get().uri("/setup").to_request();
        let resp = test::call_service(&mut app, req).await;

        // Extract the session cookie
        let cookies: Vec<_> = resp.response().cookies().collect();

        // Now call the protected endpoint with the session cookie
        let mut req_builder = test::TestRequest::get().uri("/api/protected");
        for cookie in cookies {
            req_builder = req_builder.cookie(cookie);
        }
        let req = req_builder.to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("protected data"));
    }
}
