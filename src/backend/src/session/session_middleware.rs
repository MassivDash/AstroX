use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{Key, SameSite};
use serde::{Deserialize, Serialize};
use std::env;
extern crate dotenv;

use dotenv::dotenv;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i64,
    username: String,
    password: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl User {
    pub fn authenticate(credentials: Credentials) -> Result<Self, AuthError> {
        dotenv().ok();

        let password = env::var("PASSWORD").expect("PASSWORD must be set");
        let username = env::var("USERNAME").expect("USERNAME must be set");

        if *credentials.username != username {
            return Err(AuthError::InvalidCredentials(anyhow::anyhow!(
                "Invalid credentials."
            )));
        }

        if *credentials.password != password {
            return Err(AuthError::InvalidCredentials(anyhow::anyhow!(
                "Invalid credentials."
            )));
        }

        Ok(User {
            id: 42,
            username: credentials.username,
            password: credentials.password,
        })
    }
}

pub fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
        .cookie_name("astroX".to_string())
        .cookie_domain(Some("localhost".to_string()))
        .cookie_path("/".to_string())
        .cookie_secure(false)
        .cookie_http_only(false)
        .cookie_same_site(SameSite::Lax)
        .build()
}