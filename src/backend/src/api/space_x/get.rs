use actix_web::{get, Error as ActixError, HttpResponse};
use reqwest::{get, Error};
use serde::{Deserialize, Serialize};

// response info at: https://docs.spacexdata.com/#5fcdb875-914f-4aef-a932-254397cf147a

// [
//   {
//     "id": 1,
//     "active": false,
//     "stages": 2,
//     "boosters": 0,
//     "cost_per_launch": 6700000,
//     "success_rate_pct": 40,
//     "first_flight": "2006-03-24",
//     "country": "Republic of the Marshall Islands",
//     "company": "SpaceX",
//     "wikipedia": "https://en.wikipedia.org/wiki/Falcon_1",
//     "description": "The Falcon 1 was an expendable launch system privately developed and manufactured by SpaceX during 2006-2009. On 28 September 2008, Falcon 1 became the first privately-developed liquid-fuel launch vehicle to go into orbit around the Earth.",
//     "rocket_id": "falcon1",
//     "rocket_name": "Falcon 1",
//     "rocket_type": "rocket"
//   }
// ]

#[derive(Deserialize, Serialize, Debug)]
struct Rocket {
    id: i32,
    active: bool,
    stages: i32,
    boosters: i32,
    cost_per_launch: i32,
    success_rate_pct: i32,
    first_flight: String,
    country: String,
    company: String,
    wikipedia: String,
    description: String,
    rocket_id: String,
    rocket_name: String,
    rocket_type: String,
}

#[get("/api/space-x")]

pub async fn json_get_space_x() -> Result<HttpResponse, ActixError> {
    let response = get("https://api.spacexdata.com/v3/rockets").await;

    match response {
        Ok(response) => {
            let response: Result<Vec<Rocket>, Error> = response.json().await;

            match response {
                Ok(res) => Ok(HttpResponse::Ok().json(res)),
                Err(error) => Ok(HttpResponse::InternalServerError().body(error.to_string())),
            }
        }
        Err(error) => Ok(HttpResponse::InternalServerError().body(error.to_string())),
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
