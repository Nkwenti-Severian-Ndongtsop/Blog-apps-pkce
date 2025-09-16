use anyhow::{Context, Result};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use base64::{engine::general_purpose, Engine as _};
use dashmap::DashMap;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;

use crate::auth::jwt::{create_jwt, Claims};

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub client: BasicClient,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub userinfo_url: String,
    pub logout_url: String,
    // PKCE session storage: maps state -> code_verifier
    pub pkce_sessions: Arc<DashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
    session_state: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PkceSession {
    pub code_verifier: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    pub preferred_username: String,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub realm_access: Option<RealmAccess>,
    pub resource_access: Option<ResourceAccess>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceAccess {
    #[serde(rename = "blog-client")]
    pub blog_client: Option<ClientAccess>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientAccess {
    pub roles: Vec<String>,
}

impl OAuthConfig {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        auth_url: String,
        token_url: String,
        userinfo_url: String,
        logout_url: String,
    ) -> Result<Self> {
        let client = BasicClient::new(
            ClientId::new(client_id.clone()),
            Some(ClientSecret::new(client_secret.clone())),
            AuthUrl::new(auth_url.clone()).context("Invalid auth URL")?,
            Some(TokenUrl::new(token_url.clone()).context("Invalid token URL")?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.clone()).context("Invalid redirect URL")?);

        Ok(Self {
            client,
            client_id,
            client_secret,
            redirect_uri,
            auth_url: auth_url.clone(),
            token_url: token_url.clone(),
            userinfo_url: userinfo_url.clone(),
            logout_url: format!(
                "{}/protocol/openid-connect/logout",
                auth_url.trim_end_matches("/protocol/openid-connect/auth")
            ),
            pkce_sessions: Arc::new(DashMap::new()),
        })
    }

    /// Generate PKCE code verifier and challenge
    fn generate_pkce() -> (PkceCodeVerifier, PkceCodeChallenge) {
        // Generate a random code verifier (43-128 characters)
        let code_verifier: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect();

        let code_verifier = PkceCodeVerifier::new(code_verifier);
        let code_challenge = PkceCodeChallenge::from_code_verifier_sha256(&code_verifier);

        (code_verifier, code_challenge)
    }

    pub fn authorize(&self) -> (String, String) {
        let (code_verifier, code_challenge) = Self::generate_pkce();

        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .set_pkce_challenge(code_challenge)
            .url();

        // Store the code verifier for later use in token exchange
        let state = csrf_token.secret().clone();
        self.pkce_sessions
            .insert(state.clone(), code_verifier.secret().clone());

        // Clean up old sessions (simple cleanup - remove sessions older than 10 minutes)
        // In production, you might want a more sophisticated cleanup mechanism
        if self.pkce_sessions.len() > 100 {
            // Remove half of the sessions to prevent memory bloat
            let keys_to_remove: Vec<String> = self
                .pkce_sessions
                .iter()
                .take(50)
                .map(|entry| entry.key().clone())
                .collect();
            for key in keys_to_remove {
                self.pkce_sessions.remove(&key);
            }
        }

        (auth_url.to_string(), state)
    }

    pub async fn exchange_code(&self, code: String, state: String) -> Result<(String, UserInfo)> {
        // Retrieve and remove the code verifier for this state
        let code_verifier_secret = self
            .pkce_sessions
            .remove(&state)
            .ok_or_else(|| anyhow::anyhow!("Invalid state or expired PKCE session"))?
            .1; // DashMap::remove returns Option<(K, V)>, we want the value

        let code_verifier = PkceCodeVerifier::new(code_verifier_secret);

        let token_response = self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(code_verifier)
            .request_async(async_http_client)
            .await
            .context("Failed to exchange code for token")?;

        let access_token = token_response.access_token().secret();

        // Get user info
        let client = reqwest::Client::new();
        let user_info: UserInfo = client
            .get(&self.userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await
            .context("Failed to request user info")?
            .json()
            .await
            .context("Failed to parse user info")?;

        Ok((access_token.to_string(), user_info))
    }
}

// Handler for initiating OAuth login
pub async fn login_handler(State(oauth_config): State<Arc<OAuthConfig>>) -> impl IntoResponse {
    let (auth_url, _) = oauth_config.authorize();
    Redirect::to(&auth_url).into_response()
}

// Handler for initiating logout
pub async fn logout_handler(State(oauth_config): State<Arc<OAuthConfig>>) -> impl IntoResponse {
    let logout_url = format!(
        "{}?post_logout_redirect_uri={}",
        oauth_config.logout_url,
        urlencoding::encode("http://10.216.68.222/")
    );

    // Clear any session cookies
    let cookie = axum::http::HeaderValue::from_str(
        "token=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT; HttpOnly; SameSite=Lax",
    )
    .unwrap();

    let mut response = axum::response::Redirect::temporary(&logout_url).into_response();

    response
        .headers_mut()
        .insert(axum::http::header::SET_COOKIE, cookie);

    response
}

// Handler for OAuth callback
pub async fn callback_handler(
    State(oauth_config): State<Arc<OAuthConfig>>,
    Query(query): Query<AuthRequest>,
) -> Result<impl IntoResponse, String> {
    // Exchange the authorization code for tokens
    let (access_token, user_info) = oauth_config
        .exchange_code(query.code, query.state)
        .await
        .map_err(|e| format!("Failed to exchange code: {}", e))?;

    // Create JWT with user info
    let mut roles = user_info
        .realm_access
        .as_ref()
        .map(|ra| ra.roles.clone())
        .unwrap_or_default();

    // Automatically assign "author" role to all authenticated users
    // This ensures that all Keycloak users have the author role for our blog
    if !roles.contains(&"author".to_string()) {
        roles.push("author".to_string());
        println!("Assigned 'author' role to user: {}", user_info.sub);
    }

    // Clone the sub field to avoid moving user_info
    let sub = user_info.sub.clone();

    let claims = Claims { sub, roles };

    let token = create_jwt(&claims).map_err(|e| e.to_string())?;

    // Create a redirect response with the token as a URL fragment
    let redirect_url = format!("/#token={}&access_token={}", token, access_token);

    // Redirect to the frontend with the token
    Ok(axum::response::Redirect::to(&redirect_url).into_response())
}
