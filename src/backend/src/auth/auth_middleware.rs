/// This module contains the implementation of the `Authentication` middleware.
///
/// The `Authentication` middleware is responsible for authenticating incoming requests based on a set of routes.
/// It checks if the requested path matches any of the specified routes and validates the session associated with the request.
/// If the path matches and the session is valid, the request is passed to the underlying service.
/// Otherwise, an unauthorized response is returned.
///
/// # Example
///
/// ```rust
/// use actix_web::{web, App};
/// use actix_session::CookieSession;
/// use crate::auth::auth_middleware::Authentication;
///
/// let routes = vec!["/protected/*".to_string()];
///
/// let app = App::new()
///     .wrap(CookieSession::signed(&[0; 32]).secure(false))
///     .wrap(Authentication { routes })
///     .service(web::resource("/protected/resource").to(handler));
/// ```
use actix_session::{Session, SessionExt};
use pin_project::pin_project;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use globset::{Glob, GlobSetBuilder};

use actix_utils::future::{ok, Either, Ready};
use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    Error, HttpResponse,
};
use futures::{ready, Future};

use crate::session::validate_session::validate_session;
pub struct Authentication {
    pub routes: Vec<String>,
}
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service,
            routes: self.routes.clone(),
        })
    }
}
pub struct AuthenticationMiddleware<S> {
    service: S,
    routes: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<EitherBody<B>>;

    type Error = Error;

    type Future = Either<AuthenticationFuture<S, B>, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session: Session = req.get_session();
        println!("{:?}", session.entries());
        let auth = validate_session(&session);
        let routes: Vec<String> = self.routes.iter().map(|s| s.to_string()).collect();
        if match_glob_patterns(routes, req.path()) {
            if auth.is_ok() {
                Either::left(AuthenticationFuture {
                    fut: self.service.call(req),
                    _phantom: PhantomData,
                })
            } else {
                let res = HttpResponse::with_body(StatusCode::UNAUTHORIZED, "Login or get away");
                Either::right(ok(req
                    .into_response(res)
                    .map_into_boxed_body()
                    .map_into_right_body()))
            }
        } else {
            Either::left(AuthenticationFuture {
                fut: self.service.call(req),
                _phantom: PhantomData,
            })
        }
    }
}

#[pin_project]
pub struct AuthenticationFuture<S, B>
where
    S: Service<ServiceRequest>,
{
    #[pin]
    fut: S::Future,
    _phantom: PhantomData<B>,
}

impl<S, B> Future for AuthenticationFuture<S, B>
where
    B: MessageBody,
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    type Output = Result<ServiceResponse<EitherBody<B>>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = match ready!(self.project().fut.poll(cx)) {
            Ok(res) => res,
            Err(err) => return Poll::Ready(Err(err)),
        };

        Poll::Ready(Ok(res.map_into_left_body()))
    }
}

fn match_glob_patterns(patterns: Vec<String>, path: &str) -> bool {
    let mut builder = GlobSetBuilder::new();

    for pattern in patterns {
        if let Ok(glob) = Glob::new(pattern.as_str()) {
            builder.add(glob);
        } else {
            panic!("Failed to create glob pattern");
        }
    }
    let set = builder.build().expect("Failed to build glob set");
    !set.matches(path).is_empty()
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use crate::cors::get_cors_options::get_cors_options;
    use crate::session::flash_messages::set_up_flash_messages;
    use crate::session::session_middleware::session_middleware;
    use crate::ssr_routes::post_login::{post_login, FormData};
    use actix_web::middleware::{NormalizePath, TrailingSlash};
    use actix_web::web::Form;
    use actix_web::{test, web, App, Responder};

    #[test]
    async fn test_match_glob_patterns() {
        // Test matching pattern
        let patterns = vec!["/auth/*".to_string(), "/posts/*".to_string()];
        assert_eq!(match_glob_patterns(patterns.clone(), "/auth/auth"), true);
        assert_eq!(match_glob_patterns(patterns.clone(), "/auth/"), true);

        let patterns = vec!["*/auth/*".to_string(), "*/posts/*".to_string()];
        assert_eq!(match_glob_patterns(patterns.clone(), "/auth/auth"), true);
        assert_eq!(match_glob_patterns(patterns.clone(), "/auth/"), true);

        assert_eq!(
            match_glob_patterns(patterns.clone(), "/auth/protected"),
            true
        );
        assert_eq!(match_glob_patterns(patterns.clone(), "/posts/456"), true);
        assert_eq!(match_glob_patterns(patterns.clone(), "/login"), false);

        // Test non-matching pattern
        let patterns = vec!["/users/*".to_string(), "/posts/*".to_string()];
        assert_eq!(
            match_glob_patterns(patterns.clone(), "/comments/789"),
            false
        );

        // Test empty patterns
        let patterns: Vec<String> = vec![];
        assert_eq!(match_glob_patterns(patterns, "/users/123"), false);

        // Test invalid glob pattern
        let patterns = vec!["[".to_string(), "/posts/*".to_string()];
        assert!(std::panic::catch_unwind(|| match_glob_patterns(patterns, "/users/123")).is_err());
    }

    async fn test_route() -> impl Responder {
        HttpResponse::Ok().body("Hey there!")
    }

    #[actix_rt::test]
    async fn test_middleware_protected() {
        let routes = vec!["/test/*".to_string()];

        let mut app = test::init_service(
            App::new()
                .route("/test/test", web::get().to(test_route))
                .wrap(set_up_flash_messages())
                .wrap(session_middleware())
                .wrap(Authentication { routes: routes }),
        )
        .await;

        let req = test::TestRequest::get().uri("/test/test").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_middleware_passthrough() {
        let routes = vec!["/test/*".to_string()];

        let mut app = test::init_service(
            App::new()
                .route("/notProtected", web::get().to(test_route))
                .wrap(set_up_flash_messages())
                .wrap(session_middleware())
                .wrap(Authentication { routes: routes })
                .wrap(NormalizePath::new(TrailingSlash::Trim)),
        )
        .await;

        let req = test::TestRequest::get().uri("/notProtected").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
    #[actix_rt::test]
    async fn test_middleware_success() {
        let env = "prod".to_string();
        let routes = vec!["/test/*".to_string()];
        let cors = get_cors_options(env, String::from("https://localhost"));
        let app = test::init_service(
            App::new()
                .route("/login", web::post().to(post_login))
                .route("/test/test", web::get().to(test_route))
                .wrap(cors)
                .wrap(Authentication {
                    routes: routes.clone(),
                })
                .wrap(set_up_flash_messages())
                .wrap(session_middleware())
                .wrap(NormalizePath::new(TrailingSlash::Trim)),
        )
        .await;

        env::set_var("USERNAME", "test_user");
        env::set_var("PASSWORD", "test_password");

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

        let session_cookie = resp
            .response()
            .cookies()
            .find(|x| {
                let name = x.name();
                name == "astroX"
            })
            .unwrap();
        let req2 = test::TestRequest::get()
            .uri("/test/test")
            .cookie(session_cookie)
            .to_request();
        let resp2 = test::call_service(&app, req2).await;
        println!("{:?}", &resp2);
        assert!(resp2.status().is_success())
    }
}
