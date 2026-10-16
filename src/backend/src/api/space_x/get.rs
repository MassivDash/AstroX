use actix_web::{get, Error as ActixError, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Rocket {
    pub id: i32,
    pub active: bool,
    pub stages: i32,
    pub boosters: i32,
    pub cost_per_launch: i32,
    pub success_rate_pct: i32,
    pub first_flight: String,
    pub country: String,
    pub company: String,
    pub wikipedia: String,
    pub description: String,
    pub rocket_id: String,
    pub rocket_name: String,
    pub rocket_type: String,
}

fn sample_rockets() -> Vec<Rocket> {
    vec![Rocket {
        id: 1,
        active: false,
        stages: 2,
        boosters: 0,
        cost_per_launch: 6700000,
        success_rate_pct: 40,
        first_flight: "2006-03-24".to_string(),
        country: "Republic of the Marshall Islands".to_string(),
        company: "SpaceX".to_string(),
        wikipedia: "https://en.wikipedia.org/wiki/Falcon_1".to_string(),
        description:
            "The Falcon 1 was an expendable launch system privately developed and manufactured by SpaceX."
                .to_string(),
        rocket_id: "falcon1".to_string(),
        rocket_name: "Falcon 1".to_string(),
        rocket_type: "rocket".to_string(),
    }]
}

const SPACE_X_ROCKETS_URL: &str = "https://api.spacexdata.com/v3/rockets";

/// Fetch the rockets from `url`, falling back to the sample rockets on any failure.
async fn fetch_rockets(url: &str) -> Vec<Rocket> {
    let client = match reqwest::Client::builder().user_agent("AstroX/1.0").build() {
        Ok(c) => c,
        Err(_) => return sample_rockets(),
    };

    match client.get(url).send().await {
        Ok(resp) if resp.status().is_success() => resp
            .json::<Vec<Rocket>>()
            .await
            .unwrap_or_else(|_| sample_rockets()),
        _ => sample_rockets(),
    }
}

#[get("/api/space-x")]
pub async fn json_get_space_x() -> Result<HttpResponse, ActixError> {
    Ok(HttpResponse::Ok().json(fetch_rockets(SPACE_X_ROCKETS_URL).await))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    /// Serve a single raw HTTP response on a random local port and return its url.
    fn serve_once(status_line: &str, body: &str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let response = format!(
            "HTTP/1.1 {status_line}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );

        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0; 1024];
            let _ = stream.read(&mut buf);
            stream.write_all(response.as_bytes()).unwrap();
        });

        url
    }

    #[actix_rt::test]
    async fn test_fetch_rockets_returns_the_api_rockets() {
        let mut rocket = sample_rockets().remove(0);
        rocket.id = 7;
        let body = serde_json::to_string(&vec![rocket]).unwrap();
        let url = serve_once("200 OK", &body);

        let rockets = fetch_rockets(&url).await;

        assert_eq!(rockets.len(), 1);
        assert_eq!(rockets[0].id, 7);
    }

    #[actix_rt::test]
    async fn test_fetch_rockets_falls_back_on_invalid_json() {
        let url = serve_once("200 OK", "not json");

        let rockets = fetch_rockets(&url).await;

        assert_eq!(rockets[0].id, 1);
    }

    #[actix_rt::test]
    async fn test_fetch_rockets_falls_back_on_http_error() {
        let url = serve_once("500 Internal Server Error", "[]");

        let rockets = fetch_rockets(&url).await;

        assert_eq!(rockets[0].id, 1);
    }

    #[actix_rt::test]
    async fn test_fetch_rockets_falls_back_when_unreachable() {
        // Bind then drop to get a port nothing is listening on
        let port = TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();

        let rockets = fetch_rockets(&format!("http://127.0.0.1:{port}")).await;

        assert_eq!(rockets[0].id, 1);
    }
}
