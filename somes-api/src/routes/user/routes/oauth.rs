use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use crate::{jwt::create_access_token, PgPoolConnection, model::User};
use crate::routes::user::get_user_from_mail_sqlx;
use crate::routes::UserError;
use reqwest::Client;

// --- OAuth Config ---
struct OAuthProviderConfig {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    auth_url: String,
    token_url: String,
    userinfo_url: String,
    scope: String,
}

// Hier später aus env vars oder Config laden
fn get_provider_config(provider: &str) -> OAuthProviderConfig {
    match provider {
        "google" => OAuthProviderConfig {
            client_id: std::env::var("GOOGLE_CLIENT_ID").unwrap(),
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET").unwrap(),
            redirect_uri: std::env::var("GOOGLE_REDIRECT_URI").unwrap(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            userinfo_url: "https://www.googleapis.com/oauth2/v3/userinfo".to_string(),
            scope: "email openid".to_string(),
        },
        "github" => OAuthProviderConfig {
            client_id: std::env::var("GITHUB_CLIENT_ID").unwrap(),
            client_secret: std::env::var("GITHUB_CLIENT_SECRET").unwrap(),
            redirect_uri: std::env::var("GITHUB_REDIRECT_URI").unwrap(),
            auth_url: "https://github.com/login/oauth/authorize".to_string(),
            token_url: "https://github.com/login/oauth/access_token".to_string(),
            userinfo_url: "https://api.github.com/user/emails".to_string(),
            scope: "user:email".to_string(),
        },
        "discord" => OAuthProviderConfig {
            client_id: std::env::var("DISCORD_CLIENT_ID").unwrap(),
            client_secret: std::env::var("DISCORD_CLIENT_SECRET").unwrap(),
            redirect_uri: std::env::var("DISCORD_REDIRECT_URI").unwrap(),
            auth_url: "https://discord.com/api/oauth2/authorize".to_string(),
            token_url: "https://discord.com/api/oauth2/token".to_string(),
            userinfo_url: "https://discord.com/api/users/@me".to_string(),
            scope: "email identify".to_string(),
        },
        "microsoft" => OAuthProviderConfig {
            client_id: std::env::var("MICROSOFT_CLIENT_ID").unwrap(),
            client_secret: std::env::var("MICROSOFT_CLIENT_SECRET").unwrap(),
            redirect_uri: std::env::var("MICROSOFT_REDIRECT_URI").unwrap(),
            auth_url: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
            token_url: "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
            userinfo_url: "https://graph.microsoft.com/v1.0/me".to_string(),
            scope: "openid email".to_string(),
        },
        _ => panic!("Unknown provider"),
    }
}

// --- Redirect zum OAuth Provider ---
pub async fn start_oauth(Path(provider): Path<String>) -> impl IntoResponse {
    println!("start oauths");
    let cfg = get_provider_config(&provider);
    let scope_encoded = cfg.scope.replace(' ', "+");
    let url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}",
        cfg.auth_url, cfg.client_id, cfg.redirect_uri, scope_encoded
    );
    Redirect::to(&url)
}

// --- Callback für OAuth ---
#[derive(Deserialize)]
pub struct OAuthCallbackQuery {
    code: String,
}

pub async fn oauth_callback(
    Path(provider): Path<String>,
    Query(query): Query<OAuthCallbackQuery>,
    PgPoolConnection(pg): PgPoolConnection,
) -> Result<impl IntoResponse, UserError> {
    let cfg = get_provider_config(&provider);

    // 1️⃣ Token beim Provider holen
    let client = Client::new();
    let token_resp = client.post(&cfg.token_url)
        .form(&[
            ("client_id", cfg.client_id.as_str()),
            ("client_secret", cfg.client_secret.as_str()),
            ("redirect_uri", cfg.redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
            ("code", query.code.as_str()),
        ])
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| UserError::Custom(
            axum::http::StatusCode::BAD_GATEWAY,
            format!("OAuth token request failed: {}", e),
        ))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| UserError::Custom(
            axum::http::StatusCode::BAD_GATEWAY,
            format!("OAuth token response invalid: {}", e),
        ))?;

    let access_token = token_resp
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            let err_msg = token_resp
                .get("error_description")
                .or_else(|| token_resp.get("error"))
                .and_then(|v| v.as_str())
                .unwrap_or("OAuth provider returned no access_token");
            UserError::Custom(
                axum::http::StatusCode::BAD_REQUEST,
                err_msg.to_string(),
            )
        })?;

    // 2️⃣ Userinfo abrufen
    let email = match provider.as_str() {
        "google" => {
            let info: serde_json::Value = client.get(&cfg.userinfo_url)
                .bearer_auth(access_token)
                .send().await.unwrap()
                .json().await.unwrap();
            info["email"].as_str().unwrap().to_string()
        }
        "github" => {
            let raw: serde_json::Value = client.get(&cfg.userinfo_url)
                .bearer_auth(access_token)
                .header("User-Agent", "somes-api")
                .header("Accept", "application/vnd.github.v3+json")
                .send().await
                .map_err(|e| UserError::Custom(
                    axum::http::StatusCode::BAD_GATEWAY,
                    format!("GitHub userinfo request failed: {}", e),
                ))?
                .json().await
                .map_err(|e| UserError::Custom(
                    axum::http::StatusCode::BAD_GATEWAY,
                    format!("GitHub userinfo response invalid: {}", e),
                ))?;
            let emails = raw.as_array().ok_or_else(|| {
                let msg = raw.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("GitHub returned unexpected format (expected email array)");
                UserError::Custom(axum::http::StatusCode::BAD_GATEWAY, msg.to_string())
            })?;
            emails.iter()
                .find(|e| e["primary"].as_bool().unwrap_or(false))
                .and_then(|e| e["email"].as_str())
                .ok_or_else(|| UserError::MissingEmail)?
                .to_string()
        }
        "discord" => {
            let info: serde_json::Value = client.get(&cfg.userinfo_url)
                .bearer_auth(access_token)
                .send().await.unwrap()
                .json().await.unwrap();
            info["email"].as_str().unwrap().to_string()
        }
        "microsoft" => {
            let info: serde_json::Value = client.get(&cfg.userinfo_url)
                .bearer_auth(access_token)
                .send().await.unwrap()
                .json().await.unwrap();
            info["mail"].as_str()
                .or_else(|| info["userPrincipalName"].as_str())
                .unwrap().to_string()
        }
        _ => panic!("Unknown provider"),
    };

    // 3️⃣ Prüfen oder neuen User anlegen
    let user: User = match get_user_from_mail_sqlx(&pg, &email).await.unwrap() {
        Some(u) => u,
        None => {
            let id = sqlx::query!(
                "insert into somes_user(email, is_email_hashed, is_admin) values ($1, $2, $3) returning id",
                &email, false, false
            )
            .fetch_one(&pg)
            .await.unwrap();

            User { id: id.id, email, is_email_hashed: false, is_admin: false }
        }
    };

    // 4️⃣ JWT erzeugen
    let jwt = create_access_token(user.id, user.email.clone(), user.is_admin).unwrap();

    // 5️⃣ Redirect ins Frontend mit JWT
    Ok(Redirect::to(&format!("http://localhost:5173/user?token={}", jwt.access_token)))
}