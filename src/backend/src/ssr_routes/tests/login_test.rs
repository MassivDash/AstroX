#[cfg(test)]
mod tests {
    use crate::session::session_middleware::session_middleware;
    use crate::ssr_routes::post_login::{post_login, FormData};
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
                .wrap(session_middleware()),
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
                .wrap(session_middleware()),
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

        //TODO test the flash messages somehow
    }
}
