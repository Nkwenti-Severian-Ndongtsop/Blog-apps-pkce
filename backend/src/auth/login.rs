use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::Cookie;
use cookie::Cookie as CookieParser;
use serde_json::json;
use crate::auth::{jwt::validate_token, test_token::generate_test_token};

pub async fn login() -> impl IntoResponse {
    let token = generate_test_token();

    let cookie = Cookie::build(("token", token.clone()))
        .path("/")
        .http_only(true)
        .finish();

    let mut response = Json(json!({ 
        "success": true, 
        "message": "Logged in successfully",
        "token": token  // For debugging purposes, remove in production
    })).into_response();
    
    response.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        cookie.to_string().parse().unwrap(),
    );
    response
}

pub async fn protected(headers: axum::http::HeaderMap) -> Response {
    // Extract token from cookie
    let token = headers
        .get_all(axum::http::header::COOKIE)
        .into_iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|cookie| {
            let cookie = cookie.trim();
            if cookie.starts_with("token=") {
                Some(cookie[6..].to_string())
            } else {
                None
            }
        })
        .next();

    let token = match token {
        Some(token) => token,
        None => return (StatusCode::UNAUTHORIZED, Json(json!({ 
            "success": false, 
            "message": "No token found in cookies" 
        }))).into_response(),
    };

    // Validate the token
    match validate_token(&token).await {
        Ok(claims) => {
            // Token is valid
            (StatusCode::OK, Json(json!({ 
                "success": true, 
                "message": "You are authorized",
                "user_id": claims.sub,
                "roles": claims.roles
            }))).into_response()
        },
        Err(e) => {
            // Token validation failed
            (StatusCode::UNAUTHORIZED, Json(json!({ 
                "success": false, 
                "message": format!("Invalid token: {}", e)
            }))).into_response()
        }
    }
}
