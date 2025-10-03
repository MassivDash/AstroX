use actix_session::Session;

pub fn validate_session(session: &Session) -> anyhow::Result<()> {
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);
    match user_id {
        Some(_id) => {
            // keep the user's session alive
            session.renew();
            Ok(())
        }
        None => Err(anyhow::anyhow!("Not authenticated")),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::session::flash_messages::set_up_flash_messages;
    use crate::session::session_middleware::session_middleware;
    use actix_session::Session;
    use actix_web::HttpResponse;

    use actix_web::{test, web, App, Responder};

    async fn manual_error(session: Session) -> impl Responder {
        let auth = validate_session(&session);

        if auth.is_ok() {
            HttpResponse::Ok().body("Hey there!")
        } else {
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }

    async fn manual_success(session: Session) -> impl Responder {
        let _ = session.insert("user_id", 41);
        let auth = validate_session(&session);

        if auth.is_ok() {
            HttpResponse::Ok().body("Hey there!")
        } else {
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }

    #[actix_rt::test]
    async fn test_validate_session_error() {
        let mut app = test::init_service(
            App::new()
                .route("/test", web::get().to(manual_error))
                .wrap(set_up_flash_messages())
                .wrap(session_middleware(None)),
        )
        .await;

        let req = test::TestRequest::get().uri("/test").to_request();

        let resp_m = test::call_service(&mut app, req).await;
        assert!(resp_m.status().is_server_error())
    }

    #[test]
    async fn test_validate_session() {
        let mut app = test::init_service(
            App::new()
                .route("/test", web::get().to(manual_success))
                .wrap(set_up_flash_messages())
                .wrap(session_middleware(None)),
        )
        .await;

        let req = test::TestRequest::get().uri("/test").to_request();

        let resp_m = test::call_service(&mut app, req).await;
        assert!(resp_m.status().is_success())
    }
}
