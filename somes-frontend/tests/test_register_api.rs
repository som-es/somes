use std::fmt::Debug;

use reqwest::{Method, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use somes_common_lib::{errors::SignUpError, SignUpInfo};
use somes_frontend::{SomesError, API_ROOT};

pub async fn post<B, Success, BadRequest>(
    b: &B,
    route: &str,
) -> Result<Result<Success, BadRequest>, SomesError>
where
    B: Serialize,
    Success: DeserializeOwned + Debug,
    BadRequest: DeserializeOwned,
{
    let client = reqwest::Client::new();

    let res = client
        .post(format!("http://localhost:3000/{route}"))
        .json(b)
        .send()
        .await;

    let Ok(res) = res else {
        return Err(SomesError::RequestError);
    };

    if res.status().is_success() {
        let data = res.json::<Success>().await;

        if let Ok(data) = data {
            return Ok(Ok(data));
        } else {
            return Err(SomesError::DeserializeError);
        }
    } else {
        return match res.status() {
            StatusCode::BAD_REQUEST => {
                let data = res.json::<BadRequest>().await;
                if let Ok(data) = data {
                    return Ok(Err(data));
                } else {
                    return Err(SomesError::DeserializeError);
                }
            }
            StatusCode::UNAUTHORIZED => Err(SomesError::Unauthorized),
            StatusCode::FORBIDDEN => Err(SomesError::Forbidden),
            StatusCode::NOT_FOUND => Err(SomesError::NotFound),
            StatusCode::INTERNAL_SERVER_ERROR => Err(SomesError::InternalServerError),
            StatusCode::UNPROCESSABLE_ENTITY => Err(SomesError::Unauthorized),
            _ => Err(SomesError::RequestError),
        };
    };
}

#[tokio::test]
async fn test_verify() {
    let res = reqwest::get("http://localhost:3000/verify?verify_id=36d161d6cd3aeb47dad3228e919be0654f0b43667b12964a640f0cd94b7d35ee");
}

// register

// curl -s \
//  -w '\n' \
//  -H 'Content-Type: application/json' \
//  -d '{"email":"test@email.com","username":"test","password":"supe%dsa9rsicher"}' \
//  http://localhost:3000/signup

// curl -s \
//  -w '\n' \
//  -H 'Content-Type: application/json' \
//  -d '{"username_or_email":"test@email.com","password":"supe%dsa9rsicher"}' \
//  http://localhost:3000/login

#[tokio::test]
async fn test_register() {
    let sign_up_info = SignUpInfo {
        email: "test@email.com".to_string(),
        username: "test".to_string(),
        password: "supe5431her".to_string(),
    };

    let res = post::<_, (), SignUpError>(&sign_up_info, "signup")
        .await
        .unwrap();

    println!("res: {res:?}");

    /*
    //request::<_, ()>(Method::POST, somes_common_lib::SIGNUP_ROUTE, sign_up_info).await.unwrap();

    //let url = format!("{API_ROOT}signup");
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3000/signup")
        .json(&sign_up_info)
        .send()
        .await.unwrap();

    println!("status {}", res.status());

    let res = res.json::<SignUpError>().await;

    println!("res: {res:?}");
    */
}
