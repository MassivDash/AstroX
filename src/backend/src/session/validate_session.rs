use actix_session::Session;
use actix_web::HttpResponse;

pub fn validate_session(session: &Session) -> Result<i64, HttpResponse> {
    println!("session: {:?}", session.entries());
    let user_id: Option<i64> = session.get("user_id").unwrap_or(None);
    println!("user_id: {:?}", user_id);

    match user_id {
        Some(id) => {
            // keep the user's session alive
            session.renew();
            Ok(id)
        }
        None => Err(HttpResponse::Unauthorized().json("Unauthorized")),
    }
}
