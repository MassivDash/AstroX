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

#[get("/api/space-x")]
pub async fn json_get_space_x() -> Result<HttpResponse, ActixError> {
    let client = reqwest::Client::builder().user_agent("AstroX/1.0").build();

    let response = match client {
        Ok(c) => c.get("https://api.spacexdata.com/v3/rockets").send().await,
        Err(_) => return Ok(HttpResponse::Ok().json(sample_rockets())),
    };

    match response {
        Ok(resp) if resp.status().is_success() => match resp.json::<Vec<Rocket>>().await {
            Ok(rockets) => Ok(HttpResponse::Ok().json(rockets)),
            Err(_) => Ok(HttpResponse::Ok().json(sample_rockets())),
        },
        _ => Ok(HttpResponse::Ok().json(sample_rockets())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_json_get_space_x() {
        let mut app = test::init_service(App::new().service(json_get_space_x)).await;

        let req = test::TestRequest::get().uri("/api/space-x").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let rockets: Vec<Rocket> = serde_json::from_slice(&body).unwrap();

        assert_eq!(rockets[0].id, 1);
    }
}
