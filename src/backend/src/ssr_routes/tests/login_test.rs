#[cfg(test)]
mod tests {
    use crate::session::session_middleware::session_middleware;
    use crate::ssr_routes::post_login::{post_login, FormData, LoginError};
    use crate::{session::flash_messages::set_up_flash_messages, ssr_routes::login::login_form};
    use actix_web::{test, web, App};
    use std::env;
    use web::Form;

    #[actix_rt::test]
    async fn test_login_form() {
        let app = test::init_service(
            App::new()
                .route("/login", web::get().to(login_form))
                .wrap(set_up_flash_messages()),
        )
        .await;
        let req = test::TestRequest::get().uri("/login").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_login_post() {
        env::set_var("USERNAME", "test_user");
        env::set_var("PASSWORD", "test_password");

        let app = test::init_service(
            App::new()
                .route("/login", web::post().to(post_login))
                .wrap(set_up_flash_messages())
                .wrap(session_middleware(None)),
        )
        .await;

        let form_data = FormData {
            username: String::from("test_user"),
            password: String::from("test_password"),
        };

        let form = Form(form_data);

        let req = test::TestRequest::post()
            .set_form(form)
            .uri("/login")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
    }

    #[actix_rt::test]
    async fn test_login_error() {
        env::set_var("USERNAME", "test_user");
        env::set_var("PASSWORD", "test_password");

        let app = test::init_service(
            App::new()
                .route("/login", web::post().to(post_login))
                .route("/login", web::get().to(login_form))
                .wrap(set_up_flash_messages())
                .wrap(session_middleware(None)),
        )
        .await;

        let form_data = FormData {
            username: String::from("spaceghost"),
            password: String::from("12345"),
        };

        let form = Form(form_data);

        let req = test::TestRequest::post()
            .set_form(form)
            .uri("/login")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());

        let req2 = test::TestRequest::get().uri("/login").to_request();
        let resp2 = test::call_service(&app, req2).await;
        assert!(resp2.status().is_success());
    }

    #[actix_rt::test]
    async fn test_post_login_unexpected_error() {
        let app = test::init_service(
            App::new()
                .wrap(set_up_flash_messages())
                .wrap(session_middleware(None))
                .route("/login", web::post().to(post_login)),
        )
        .await;

        // Simulate an unexpected error by providing invalid session data
        let form_data = FormData {
            username: "test_user".to_string(),
            password: "test_password".to_string(),
        };

        let req = test::TestRequest::post()
            .set_form(&form_data)
            .uri("/login")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
    }

    #[actix_rt::test]
    async fn test_failed_login_shows_the_flash_error_on_the_form() {
        env::set_var("USERNAME", "test_user");
        env::set_var("PASSWORD", "test_password");

        let app = test::init_service(
            App::new()
                .route("/login", web::post().to(post_login))
                .route("/login", web::get().to(login_form))
                .wrap(set_up_flash_messages())
                .wrap(session_middleware(None)),
        )
        .await;

        let req = test::TestRequest::post()
            .set_form(FormData {
                username: "intruder".to_string(),
                password: "wrong".to_string(),
            })
            .uri("/login")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());

        // Carry the flash cookie over to the redirect target, like a browser would
        let mut follow_up = test::TestRequest::get().uri("/login");
        for cookie in resp.response().cookies() {
            follow_up = follow_up.cookie(cookie.into_owned());
        }
        let body = test::call_and_read_body(&app, follow_up.to_request()).await;
        let html = String::from_utf8(body.to_vec()).unwrap();

        assert!(html.contains("<p><i>Authentication failed</i></p>"));
    }

    #[actix_rt::test]
    async fn test_login_error_debug_lists_the_cause_chain() {
        let error = LoginError::AuthError(anyhow::anyhow!("bad password"));

        let debug = format!("{:?}", error);

        assert!(debug.starts_with("Authentication failed\n"));
        assert!(debug.contains("Caused by:\n\tbad password"));
    }

    #[actix_rt::test]
    async fn test_unexpected_login_error_message() {
        let error = LoginError::from(anyhow::anyhow!("session store down"));

        assert_eq!(error.to_string(), "Something went wrong");
        assert!(format!("{:?}", error).contains("Caused by:\n\tsession store down"));
    }
}
