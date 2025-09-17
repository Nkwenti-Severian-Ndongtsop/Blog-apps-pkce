# OAuth2 Flows in Keycloak: Implicit vs. Modern PKCE

## 1. Implicit Flow (Legacy)

### Overview

- **Designed for SPAs (Single Page Applications)**
- **Tokens returned directly in the URL fragment**
- **No client secret required**
- **Vulnerable to token leakage and replay attacks**

### How It Works

1. **User clicks Login**
2. **App redirects to Keycloak with response_type=token**
3. **Keycloak authenticates user**
4. **Access token returned in URL fragment**
5. **App extracts token from URL**

### Security Concerns

- Tokens exposed in browser history and logs
- No refresh tokens (short-lived access)
- Susceptible to CSRF and XSS attacks

---

## 2. Authorization Code Flow with PKCE (Modern, Recommended)

### Overview

- **PKCE (Proof Key for Code Exchange) adds security for public clients**
- **Authorization code exchanged for tokens via backend**
- **Mitigates interception and replay attacks**

### How PKCE Works in This Project

#### Step-by-Step Flow

1. **User initiates login**

   - Browser app redirects to Keycloak's `/auth` endpoint with:
     - `response_type=code`
     - `client_id=blog-client`
     - `redirect_uri=http://localhost/auth/callback`
     - `code_challenge` (SHA256 hash of a random string)
     - `code_challenge_method=S256`

2. **Keycloak authenticates user**

   - User enters credentials
   - Keycloak validates and redirects to `redirect_uri` with `code`

3. **App exchanges code for tokens**

   - App sends POST request to Keycloak `/token` endpoint with:
     - `code` (from previous step)
     - `code_verifier` (original random string)
     - `client_id`
     - `redirect_uri`
   - Keycloak verifies `code_verifier` matches `code_challenge`

4. **Keycloak returns tokens**

   - Access token, ID token, and optionally refresh token

5. **App uses tokens for API requests**
   - Tokens sent in Authorization header to backend

#### Keycloak Configuration in This Project

- **Client Type:** OpenID Connect
- **PKCE Method:** S256
- **Client Authentication:** OFF (public client)
- **Standard Flow:** ON (Authorization Code Flow)
- **Redirect URI:** `http://localhost/auth/callback`

#### Rust Backend Integration

```rust
// filepath: src/main.rs
let oauth_config = OAuthConfig::new(
    "blog-client".to_string(),
    "".to_string(), // No client secret for PKCE
    "http://localhost/auth/callback".to_string(),
    "http://localhost:8080/realms/blog-realm/protocol/openid-connect/auth".to_string(),
    "http://localhost:8080/realms/blog-realm/protocol/openid-connect/token".to_string(),
    "http://localhost:8080/realms/blog-realm/protocol/openid-connect/userinfo".to_string(),
    "http://localhost:8080/realms/blog-realm/protocol/openid-connect/logout".to_string(),
)?;
```

- **Backend validates tokens from Keycloak**
- **Role-based access enforced (e.g., 'author' role)**

---

## Why PKCE Is Better

- **Prevents code interception attacks**
- **No client secret needed (safe for SPAs and mobile apps)**
- **Tokens never exposed in browser URL**
- **Refresh tokens supported**

---

## Visual Comparison

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   Browser   │◄───►│   Keycloak  │◄───►│   Backend   │
└─────────────┘      └─────────────┘      └─────────────┘
      ▲ PKCE: Secure code exchange, tokens via backend
      ▼ Implicit: Tokens in browser URL (not recommended)
```

---

## Summary

- **Implicit Flow:** Legacy, insecure, not recommended
- **PKCE Flow:** Modern, secure, recommended for all public clients
- **Keycloak:** Fully supports PKCE, easy to configure
- **This Project:** Uses PKCE for secure authentication with Rust backend

---

## References

- [OAuth2 PKCE Explained](https://oauth.net/2/pkce/)
- [Keycloak Docs: PKCE](https://www.keycloak.org/docs/latest/server_admin/#_pkce)
-
