use actix_session::Session;
use actix_web::error::InternalError;
use actix_web::http::header::ContentType;
use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;
use actix_web_flash_messages::IncomingFlashMessages;
use serde::Serialize;
use std::error::Error;
use std::fmt::Write;

use crate::session::session_middleware::{AuthError, Credentials, User};

pub async fn login_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
    {error_html}
    <form action="/login" method="post">
        <label>Username
            <input
                type="text"
                placeholder="Enter Username"
                name="username"
            >
        </label>
        <label>Password
            <input
                type="password"
                placeholder="Enter Password"
                name="password"
            >
        </label>
        <button type="submit">Login</button>
    </form>
</body>
</html>"#,
        ))
}

#[derive(serde::Deserialize, Serialize)]
pub struct FormData {
    pub username: String,
    pub password: String,
}

pub async fn login(
    form: web::Form<FormData>,
    session: Session,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = Credentials {
        username: form.0.username,
        password: form.0.password,
    };

    match User::authenticate(credentials) {
        Ok(user) => {
            println!("User authenticated: {:?}", user);
            session.renew();
            session
                .insert("user_id", user.id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;

            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "/auth/auth"))
                .finish())
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            Err(login_redirect(e))
        }
    }
}

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    FlashMessage::error(e.to_string()).send();
    let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .finish();
    InternalError::from_response(e, response)
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current: Option<&dyn Error> = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::session::flash_messages::set_up_flash_messages;
    use crate::session::session_middleware::session_middleware;
    use actix_web::{test, App};
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
                .route("/login", web::post().to(login))
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
                .route("/login", web::post().to(login))
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
