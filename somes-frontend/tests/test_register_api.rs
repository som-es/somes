use reqwest::{Method, StatusCode};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use somes_common_lib::{SignUpInfo, errors::SignUpError};
use somes_frontend::{API_ROOT, SomesError};


pub async fn post<B: Serialize, R: DeserializeOwned, C: DeserializeOwned>(b: &B, route: &str) -> Result<Result<R, C>, SomesError> {
    let client = reqwest::Client::new();
    
    let res = client.post(format!("http://localhost:3000/{route}"))
        .json(b)
        .send()
        .await;

    let Ok(res) = res else {
        return Err(SomesError::RequestError);
    };

    if res.status().is_success() {
        let data = res.json::<R>().await;

        if let Ok(data) = data {
            return Ok(Ok(data));
        } else {
            return Err(SomesError::DeserializeError);
        }
    } else {
        return match res.status() {
            StatusCode::BAD_REQUEST => {
                let data = res.json::<C>().await;
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
            StatusCode::UNPROCESSABLE_ENTITY => {
                Err(SomesError::Unauthorized)
            }
            _ => Err(SomesError::RequestError),
        }
    };
}

#[tokio::test]
async fn test_register() {
    let sign_up_info = SignUpInfo {
        email: "test@email.com".to_string(),
        username: "test".to_string(),
        password: "supersicher".to_string(),
    };
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
}