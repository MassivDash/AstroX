use actix_session::Session;
use actix_web::error::InternalError;
use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;
use serde::Serialize;
use std::error::Error;

use crate::session::session_middleware::{AuthError, Credentials, User};

#[derive(serde::Deserialize, Serialize)]
pub struct FormData {
    pub username: String,
    pub password: String,
}

pub async fn post_login(
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
            match session.insert("user_id", user.id) {
                Ok(_) => Ok(HttpResponse::SeeOther()
                    .insert_header((LOCATION, "/auth/protected"))
                    .finish()),
                Err(e) => {
                    let error_message = format!("Failed to insert user_id into session: {}", e);
                    Err(login_redirect(LoginError::UnexpectedError(
                        anyhow::anyhow!(error_message),
                    )))
                }
            }
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
