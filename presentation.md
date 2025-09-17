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

1. **User clicks Login button in browser**

   **Frontend (Browser JavaScript):**
   - Redirects browser to backend endpoint: `GET /auth/login`

   **Backend (Rust Server):**
   - Receives the login request
   - Generates random 128-character code verifier (alphanumeric)
   - Creates SHA256 hash of code verifier as code challenge
   - Stores code verifier mapped to state in DashMap (in-memory)
   - Builds Keycloak authorization URL with:
     - `response_type=code`
     - `client_id=blog-client`
     - `redirect_uri=http://localhost/auth/callback`
     - `code_challenge` (SHA256 hash of code verifier)
     - `code_challenge_method=S256`
     - `scope=openid profile email`
   - Redirects browser to Keycloak authorization URL

2. **Keycloak handles authentication**

   **Keycloak:**
   - Displays login form to user
   - User enters credentials
   - Keycloak validates credentials and user session
   - Keycloak redirects browser back to backend with:
     - `code` (authorization code)
   - Redirect goes to: `GET /auth/callback?code=...&state=...`

3. **Backend exchanges code for tokens**

   **Backend (Rust Server):**
   - Receives callback: `GET /auth/callback?code=...&state=...`
   - Retrieves stored `code_verifier` from DashMap using the `state`
   - Removes code verifier from DashMap (one-time use)
   - Sends POST request to Keycloak `/token` endpoint with:
     - `grant_type=authorization_code`
     - `code` (authorization code from step 2)
     - `code_verifier` (original 128-character random string)
     - `client_id=blog-client`
     - `redirect_uri=http://localhost/auth/callback`
   
   **Keycloak:**
   - Verifies `code_verifier` SHA256 hash matches stored `code_challenge`
   - Validates authorization code hasn't been used before

4. **Keycloak returns tokens to backend**

   **Keycloak:**
   - Responds with JSON containing:
     - `access_token` (JWT with user claims and roles)
     - `id_token` (OpenID Connect identity token)
     - `refresh_token` (for token renewal)
     - `token_type=Bearer`
     - `expires_in` (token lifetime in seconds)

5. **Frontend (Browser):**
   - Receives redirect to home page
   - Cookie is automatically stored by browser
   - Calls `GET /auth/status` to check authentication
   - Updates UI to show logout button and admin features
   - For subsequent API requests, authentication works via:
   - HttpOnly cookie (automatically included by browser) 


PKCE Flow:
1. Browser → Backend: User clicks login
2. Backend → Keycloak: Auth request + code_challenge
3. Keycloak → Browser: Login form, then redirect with authorization code
4. Browser → Backend: Authorization code via callback
5. Backend → Keycloak: Code + code_verifier → Tokens
6. Backend → Browser: Set secure cookie + redirect to home

Implicit Flow (Legacy):
1. Browser → Keycloak: Auth request
2. Keycloak → Browser: Tokens in URL (vulnerable)
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
