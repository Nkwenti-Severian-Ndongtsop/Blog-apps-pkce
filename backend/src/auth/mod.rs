#![allow(warnings)]
use anyhow::Result;
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{Json, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod jwt;
pub mod login;
pub mod oauth;
pub mod test_token;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
}

/// Authentication middleware for Axum
pub async fn auth_middleware(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    println!("Auth middleware called");

    // Try to get token from Authorization header first
    let token = if let Some(auth_header) = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        println!("Found token in Authorization header");
        jwt::extract_token_from_header(auth_header)
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": "Invalid Authorization header format",
                        "message": "Expected 'Bearer <token>'"
                    })),
                )
            })
            .map(String::from)?
    }
    // If no Authorization header, try to get token from cookie
    else if let Some(cookie_header) = headers
        .get(axum::http::header::COOKIE)
        .and_then(|h| h.to_str().ok())
    {
        println!("Checking for token in cookies");
        // Parse cookies
        let cookies: Vec<&str> = cookie_header.split(';').map(|c| c.trim()).collect();

        // Find the token cookie
        let token_cookie = cookies
            .iter()
            .find(|c| c.starts_with("token="))
            .map(|c| &c[6..]); // Remove 'token=' prefix

        match token_cookie {
            Some(token) => {
                println!("Found token in cookie");
                token.to_string()
            }
            None => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": "No authentication token found",
                        "message": "Please log in first"
                    })),
                ))
            }
        }
    } else {
        // No token found in either location
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Missing authentication",
                "message": "Please provide a Bearer token or log in first"
            })),
        ));
    };

    // Extract and validate token
    let claims = jwt::validate_token(&token).await.map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Invalid or expired token",
                "message": "Please provide a valid Bearer token"
            })),
        )
    })?;

    println!("Token validated successfully. Roles: {:?}", claims.roles);

    // Check if user has author role
    if !claims.roles.contains(&"author".to_string()) {
        println!("User does not have author role");
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "Insufficient permissions",
                "message": "You need the 'author' role to access this endpoint"
            })),
        ));
    }

    println!("User has author role, proceeding with request");

    // Add claims to request extensions for use in handlers
    let mut request = request;
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Extract claims from request extensions
pub fn extract_claims(request: &Request) -> Option<&Claims> {
    let claims = request.extensions().get::<Claims>();
    if claims.is_some() {
        println!("Claims found in request extensions");
    } else {
        println!("No claims found in request extensions");
    }
    claims
}

/// Check if user has specific role
pub fn has_role(claims: &Claims, role: &str) -> bool {
    claims.roles.contains(&role.to_string())
}
