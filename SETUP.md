# Blog Apps - Local Development Setup

This guide will help you set up the Blog Apps project for local development using a hybrid approach:
- **Keycloak + PostgreSQL**: Running in Docker containers
- **Rust Backend**: Running locally with `cargo run`
- **Nginx**: Running in Docker, proxying to local backend

## Prerequisites

Make sure you have the following installed:
- [Docker](https://docs.docker.com/get-docker/) and Docker Compose
- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/downloads)

## Quick Start

### 1. Clone and Setup Environment

```bash
git clone <your-repo-url>
cd Blog-apps
cp env.example .env
```

### 2. Start Docker Services (Keycloak + PostgreSQL + Nginx)

```bash
docker compose --env-file .env up -d
```

This will start:
- **PostgreSQL**: `localhost:5433` (Keycloak database)
- **Keycloak**: `localhost:8080` (Authentication server)
- **Nginx**: `localhost:80` (Reverse proxy)

### 3. Start Rust Backend

```bash
cd backend
cargo run
```

The Rust backend will start on `localhost:8000`.

### 4. Verify Services

Check that all services are running:

```bash
# Check Docker services
docker compose ps

# Check backend is running
curl http://localhost:8000/health

# Check nginx is proxying correctly
curl http://localhost/health
```

## Keycloak Configuration

### Access Keycloak Admin Console

1. Open your browser and go to: `http://localhost:8080`
2. Click on **Administration Console**
3. Login with:
   - **Username**: `admin`
   - **Password**: `admin`

### Create a Realm

1. Click on the dropdown next to **Master** (top left)
2. Click **Create Realm**
3. Set **Realm name**: `blog-realm`
4. Click **Create**

### Create a Client

1. In the left sidebar, click **Clients**
2. Click **Create client**
3. Configure the client:
   - **Client type**: `OpenID Connect`
   - **Client ID**: `blog-client`
   - Click **Next**
4. Configure capabilities:
   - **Client authentication**: `OFF` (for PKCE)
   - **Authorization**: `OFF`
   - **Standard flow**: `ON` (Authorization Code Flow)
   - **Direct access grants**: `OFF`
   - Click **Next**
5. Configure login settings:
   - **Root URL**: `http://localhost`
   - **Home URL**: `http://localhost`
   - **Valid redirect URIs**: 
     - `http://localhost/auth/callback`
     - `http://localhost/*`
   - **Valid post logout redirect URIs**: `http://localhost`
   - **Web origins**: `http://localhost`
   - **Admin URL**: Leave empty
   - Click **Save**

### ⚠️ **Important: Fix Keycloak Base URL Issue**

If you're getting "Unable to connect" errors or redirects to wrong ports, you need to configure Keycloak's base URL:

1. In Keycloak Admin Console, go to **Realm Settings**
2. Click on the **General** tab
3. Set **Frontend URL**: `http://localhost:8080`
4. Click **Save**

This ensures Keycloak uses the correct port (8080) for all authentication endpoints.

### Configure Client for PKCE

1. In the **Capability config** step (as shown in your screenshot):
   - Set **PKCE Method**: Select `S256` from the dropdown
   - Ensure **Standard flow** is `ON` (Authorization Code Flow)
   - Ensure **Client authentication** is `OFF` (for public PKCE clients)
2. Click **Next** and then **Save**

**Important**: The PKCE Method dropdown must be set to `S256` for the secure code challenge method to work properly with our Rust backend implementation.

### Create a User

1. In the left sidebar, click **Users**
2. Click **Create new user**
3. Fill in user details:
   - **Username**: `testuser`
   - **Email**: `test@example.com`
   - **First name**: `Test`
   - **Last name**: `User`
   - **Email verified**: `ON`
   - **Enabled**: `ON`
4. Click **Create**
5. Go to the **Credentials** tab
6. Click **Set password**
7. Set password: `password123`
8. Set **Temporary**: `OFF`
9. Click **Save**

### Create Author Role

1. In the left sidebar, click **Realm roles**
2. Click **Create role**
3. Set **Role name**: `author`
4. Set **Description**: `Blog author role`
5. Click **Save**

### Assign Role to User

1. Go to **Users** → Select your user (`testuser`)
2. Click on the **Role mapping** tab
3. Click **Assign role**
4. Select **author** role
5. Click **Assign**

## Environment Configuration

Update your `.env` file if needed:

```env
# PostgreSQL Configuration
POSTGRES_CONTAINER_NAME=kc-postgres
POSTGRES_DB=keycloak
POSTGRES_USER=keycloak
POSTGRES_PASSWORD=keycloak
POSTGRES_PORT=5433
POSTGRES_HOST=keycloak-db
POSTGRES_DATA_PATH=./postgres-data

# Keycloak Configuration
KEYCLOAK_CONTAINER_NAME=keycloak
KEYCLOAK_ADMIN=admin
KEYCLOAK_ADMIN_PASSWORD=admin
KEYCLOAK_PORT=8080
KEYCLOAK_HOSTNAME=localhost

# Nginx Configuration
NGINX_CONTAINER_NAME=nginx
NGINX_PORT=80
```

## Backend Configuration

The Rust backend needs to be configured to work with your Keycloak setup. Update the backend configuration (typically in `src/main.rs` or a config file) with:

```rust
// OAuth Configuration
let oauth_config = OAuthConfig::new(
    "blog-client".to_string(),                                    // client_id
    "".to_string(),                                              // client_secret (empty for PKCE)
    "http://localhost/auth/callback".to_string(),                // redirect_uri
    "http://localhost:8080/realms/blog-realm/protocol/openid-connect/auth".to_string(), // auth_url
    "http://localhost:8080/realms/blog-realm/protocol/openid-connect/token".to_string(), // token_url
    "http://localhost:8080/realms/blog-realm/protocol/openid-connect/userinfo".to_string(), // userinfo_url
    "http://localhost:8080/realms/blog-realm/protocol/openid-connect/logout".to_string(), // logout_url
)?;
```

## Testing the Setup

### 1. Test Authentication Flow

1. Open your browser and go to: `http://localhost`
2. Try to access a protected endpoint (this should redirect to Keycloak)
3. Login with:
   - **Username**: `testuser`
   - **Password**: `password123`
4. You should be redirected back to the application with authentication

### 2. Test API Endpoints

```bash
# Health check
curl http://localhost/health

# Public endpoints
curl http://localhost/api/posts

# Protected endpoints (requires authentication)
curl http://localhost/admin/posts
```

## Development Workflow

### Starting Development

```bash
# Start Docker services
docker compose up -d

# Start backend in development mode
cd backend
cargo run
```

### Stopping Services

```bash
# Stop Docker services
docker compose down

# Stop backend (Ctrl+C in the terminal where cargo run is running)
```

### Viewing Logs

```bash
# Docker services logs
docker compose logs -f

# Backend logs (visible in the terminal where cargo run is running)
```

## Troubleshooting

### Port Conflicts

If you encounter port conflicts:

1. **PostgreSQL (5433)**: Change `POSTGRES_PORT` in `.env`
2. **Keycloak (8080)**: Change `KEYCLOAK_PORT` in `.env`
3. **Nginx (80)**: Change `NGINX_PORT` in `.env`
4. **Backend (8000)**: Update the port in your Rust backend code

### Keycloak Not Accessible

1. Check if Keycloak container is running: `docker compose ps`
2. Check Keycloak logs: `docker compose logs keycloak`
3. Wait a few minutes for Keycloak to fully start up

### Backend Connection Issues

1. Ensure the backend is running on port 8000
2. Check that `host.docker.internal` resolves correctly
3. On Linux, you might need to use `172.17.0.1` instead of `host.docker.internal`

### CORS Issues

If you encounter CORS issues:

1. Check that your frontend origin is included in Keycloak client settings
2. Verify nginx CORS headers are properly configured
3. Ensure the backend CORS configuration matches your frontend URL

## Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Browser       │    │   Nginx         │    │   Rust Backend  │
│   localhost     │◄──►│   localhost:80  │◄──►│   localhost:8000│
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐    ┌─────────────────┐
                       │   Keycloak      │◄──►│   PostgreSQL    │
                       │   localhost:8080│    │   localhost:5433│
                       └─────────────────┘    └─────────────────┘
```

## Security Features

- **PKCE (Proof Key for Code Exchange)**: Enhanced security for OAuth2 flows
- **JWT Token Validation**: Backend validates tokens from Keycloak
- **Role-based Access Control**: Users need 'author' role for protected endpoints
- **CORS Protection**: Properly configured cross-origin resource sharing
- **Secure Headers**: Nginx configured with security headers

## Next Steps

1. **Frontend Integration**: Add your frontend application
2. **Database Integration**: Connect your backend to a separate application database
3. **Production Deployment**: Configure for production environment
4. **SSL/TLS**: Add HTTPS support for production
5. **Monitoring**: Add logging and monitoring solutions

## Support

If you encounter issues:

1. Check the logs: `docker compose logs`
2. Verify all services are running: `docker compose ps`
3. Test individual components separately
4. Check the Keycloak admin console for configuration issues

Happy coding! 🚀
