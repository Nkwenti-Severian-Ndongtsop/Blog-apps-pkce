#![allow(warnings)]

use crate::auth::{
    auth_middleware, 
    jwt::validate_token, 
    oauth::{callback_handler, login_handler, logout_handler, OAuthConfig},
};
use axum::http::HeaderName;
use axum::{
    extract::{Form, Path, Query, State},
    http::{header, HeaderMap, Method, StatusCode},
    middleware,
    response::{Html, Json, Response},
    routing::{delete, get, post, put},
    Router,
};
use serde_json::json;
use std::net::SocketAddr;
use std::{env, fs, path::Path as StdPath, sync::Arc, time::Duration};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod auth;
mod markdown;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 Starting blog backend server with Axum and Keycloak auth...");

    // Configure OAuth with PKCE (no client secret needed for public clients)
    let oauth_config = OAuthConfig::new(
        "blog-client".to_string(),
        "".to_string(), // Empty client secret for PKCE public client
        "http://localhost/auth/callback".to_string(), // This will be handled by Nginx
        "http://localhost:8080/realms/blog-realm/protocol/openid-connect/auth".to_string(),
        "http://localhost:8080/realms/blog-realm/protocol/openid-connect/token".to_string(),
        "http://localhost:8080/realms/blog-realm/protocol/openid-connect/userinfo".to_string(),
        "http://localhost:8080/realms/blog-realm/protocol/openid-connect/logout".to_string(),
    )?;
    let oauth_config = Arc::new(oauth_config);

    // Get port from environment or use default
    let port = env::var("BLOG_SERVICE_PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000);

    // Create CORS layer with proper configuration for credentials and headers
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
            header::COOKIE,
            header::HeaderName::from_static("x-requested-with"),
        ])
        .expose_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::CONTENT_LENGTH,
            header::HeaderName::from_static("x-auth-token"),
        ])
        .allow_origin([
            "http://localhost".parse().unwrap(),
            "http://localhost:80".parse().unwrap(),
            "http://localhost:8080".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    // Build our application with routes
    let app = Router::new()
        // Auth routes
        .route("/auth/login", get(login_handler))
        .route("/auth/callback", get(callback_handler))
        .route("/auth/logout", get(logout_handler))


        // API routes
        .route("/health", get(health_check))
        .route("/auth/status", get(auth_status))
        .route("/posts", get(list_posts))
        .route("/posts/{slug}", get(get_post))
        .route("/preview", post(preview_markdown))
        // Admin routes
        .route("/admin/new", get(serve_new_post))
        .route("/admin/edit/{slug}", get(serve_edit_post))
        // Frontend routes
        .route("/", get(serve_index))
        .route("/static/{file}", get(serve_static))
        .route("/favicon.ico", get(serve_favicon))
        .route("/posts/html", get(serve_posts_html))
        .nest(
            "/admin",
            Router::new()
                .route("/new", post(create_post))
                .route("/edit/{slug}", put(edit_post))
                .route("/delete/{slug}", delete(delete_post))
                .layer(middleware::from_fn(auth::auth_middleware)),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(oauth_config);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("📡 Server running on http://{}:{}", "0.0.0.0", port);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// Authentication status endpoint
async fn auth_status(headers: HeaderMap) -> Json<serde_json::Value> {
    // Try to get token from Authorization header or cookie
    let token = if let Some(auth_header) = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        auth::jwt::extract_token_from_header(auth_header)
            .map(String::from)
            .ok()
    } else if let Some(cookie_header) = headers
        .get(axum::http::header::COOKIE)
        .and_then(|h| h.to_str().ok())
    {
        // Parse cookies to find token
        let cookies: Vec<&str> = cookie_header.split(';').map(|c| c.trim()).collect();
        cookies
            .iter()
            .find(|c| c.starts_with("token="))
            .map(|c| c[6..].to_string()) // Remove 'token=' prefix
    } else {
        None
    };

    match token {
        Some(token) => {
            // Validate the token
            match auth::jwt::validate_token(&token).await {
                Ok(claims) => {
                    Json(json!({
                        "authenticated": true,
                        "user": {
                            "id": claims.sub,
                            "roles": claims.roles
                        }
                    }))
                }
                Err(_) => {
                    Json(json!({
                        "authenticated": false,
                        "error": "Invalid or expired token"
                    }))
                }
            }
        }
        None => {
            Json(json!({
                "authenticated": false,
                "error": "No authentication token found"
            }))
        }
    }
}

async fn list_posts() -> Json<serde_json::Value> {
    match std::fs::read_to_string("posts.json") {
        Ok(content) => match serde_json::from_str::<Vec<crate::markdown::Post>>(&content) {
            Ok(posts) => {
                let post_summaries: Vec<serde_json::Value> = posts
                    .iter()
                    .map(|post| {
                        json!({
                            "slug": post.slug,
                            "title": post.title,
                            "author": post.author,
                            "created_at": post.created_at,
                            "updated_at": post.updated_at
                        })
                    })
                    .collect();

                Json(json!({
                    "success": true,
                    "posts": post_summaries
                }))
            }
            Err(_) => Json(json!({
                "success": false,
                "error": "Failed to parse posts.json"
            })),
        },
        Err(_) => Json(json!({
            "success": false,
            "error": "Failed to read posts.json"
        })),
    }
}

async fn get_post(Path(slug): Path<String>) -> Result<Html<String>, StatusCode> {


    // Try to get post data first
    match crate::markdown::reader::read_post(&slug) {
        Ok(post) => {
            // Read the post template
            match std::fs::read_to_string("../frontend/templates/post.html") {
                Ok(mut template) => {
                    // Simple template replacement
                    template = template.replace("{{ title }}", &post.title);
                    template = template.replace("{{ author }}", &post.author);
                    template = template.replace(
                        "{{ created_at }}",
                        &post.created_at.format("%B %d, %Y").to_string(),
                    );
                    template = template.replace(
                        "{{ updated_at }}",
                        &post.updated_at.format("%B %d, %Y").to_string(),
                    );

                    // Get the rendered content
                    match crate::markdown::reader::read_and_render_markdown(&slug) {
                        Ok(html_content) => {
                            template = template.replace("{{ content | safe }}", &html_content);

                            Ok(Html(template))
                        }
                        Err(_) => {
                            template = template
                                .replace("{{ content | safe }}", "<p>Error loading content.</p>");
                            Ok(Html(template))
                        }
                    }
                }
                Err(_) => {
                    // Fallback to simple HTML if template not found
                    match crate::markdown::reader::read_and_render_markdown(&slug) {
                        Ok(html_content) => {
                            let simple_html = format!(
                                "<!DOCTYPE html><html><head><title>{}</title></head><body><h1>{}</h1><div>{}</div></body></html>",
                                post.title, post.title, html_content
                            );
                            Ok(Html(simple_html))
                        }
                        Err(_) => Ok(Html(
                            "<h1>Post not found</h1><p>The requested post could not be found.</p>"
                                .to_string(),
                        )),
                    }
                }
            }
        }
        Err(_) => {

            Ok(Html(
                "<h1>Post not found</h1><p>The requested post could not be found.</p>".to_string(),
            ))
        }
    }
}

#[derive(serde::Deserialize)]
struct PreviewRequest {
    content: String,
}

async fn preview_markdown(Json(payload): Json<PreviewRequest>) -> Html<String> {
    let html_content = crate::markdown::reader::markdown_to_html(&payload.content);
    Html(html_content)
}

#[derive(serde::Deserialize)]
struct CreatePostRequest {
    title: String,
    content: String,
}

#[derive(serde::Serialize)]
struct AdminResponse {
    success: bool,
    message: String,
    slug: Option<String>,
}

async fn create_post(
    Form(payload): Form<CreatePostRequest>,
) -> Result<Json<AdminResponse>, StatusCode> {
    // Authentication is handled by middleware; claims should be available in request extensions
    // For now, we'll use a placeholder author - in a real implementation,
    // we'd extract the claims from the middleware
    let slug = crate::utils::generate_unique_slug(&payload.title);

    let post = crate::markdown::Post {
        slug: slug.clone(),
        title: payload.title,
        author: "placeholder-author".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        content: payload.content,
    };

    // Save the post
    match crate::markdown::writer::create_post(&post) {
        Ok(_) => {

            Ok(Json(AdminResponse {
                success: true,
                message: "Post created successfully".to_string(),
                slug: Some(slug),
            }))
        }
        Err(e) => {

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(serde::Deserialize)]
struct UpdatePostRequest {
    title: String,
    content: String,
}

async fn edit_post(
    Path(slug): Path<String>,
    Form(payload): Form<UpdatePostRequest>,
) -> Result<Json<AdminResponse>, StatusCode> {
    // Load existing post to preserve author and created_at
    let existing_post = match crate::markdown::reader::read_post(&slug) {
        Ok(p) => p,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    let post = crate::markdown::Post {
        slug: slug.clone(),
        title: payload.title,
        author: existing_post.author,
        created_at: existing_post.created_at,
        updated_at: chrono::Utc::now(),
        content: payload.content,
    };

    // Update the post
    match crate::markdown::writer::update_post(&post) {
        Ok(_) => {

            Ok(Json(AdminResponse {
                success: true,
                message: "Post updated successfully".to_string(),
                slug: Some(slug),
            }))
        }
        Err(e) => {

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn delete_post(Path(slug): Path<String>) -> Result<Json<AdminResponse>, StatusCode> {
    // Authentication is handled by middleware

    match crate::markdown::writer::delete_post(&slug) {
        Ok(_) => {

            Ok(Json(AdminResponse {
                success: true,
                message: "Post deleted successfully".to_string(),
                slug: Some(slug),
            }))
        }
        Err(e) => {

            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Template serving functions
async fn serve_index() -> Html<String> {
    match std::fs::read_to_string("../frontend/templates/index.html") {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>Error</h1><p>Could not load index template.</p>".to_string()),
    }
}

async fn serve_new_post() -> Html<String> {
    match std::fs::read_to_string("../frontend/templates/admin/new.html") {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>Error</h1><p>Could not load new post template.</p>".to_string()),
    }
}

async fn serve_edit_post(Path(slug): Path<String>) -> Html<String> {
    // First try to get the existing post data
    match crate::markdown::reader::read_post(&slug) {
        Ok(post) => {
            // Read the template
            match std::fs::read_to_string("../frontend/templates/admin/edit.html") {
                Ok(mut template) => {
                    // Simple template replacement (in a real app, use a proper templating engine)
                    template = template.replace("{{ slug }}", &slug);
                    template = template.replace("{{ title }}", &post.title);
                    template = template.replace("{{ content }}", &post.content);
                    Html(template)
                }
                Err(_) => Html("<h1>Error</h1><p>Could not load edit template.</p>".to_string()),
            }
        }
        Err(_) => Html("<h1>Error</h1><p>Post not found.</p>".to_string()),
    }
}

async fn serve_static(Path(file): Path<String>) -> Result<Response, StatusCode> {
    // Use absolute path from project root to avoid issues with relative paths
    let current_dir = std::env::current_dir()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let project_root = current_dir.parent()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let file_path = project_root.join("frontend").join("static").join(&file);

    match std::fs::read(&file_path) {
        Ok(content) => {
            let content_type = if file.ends_with(".css") {
                "text/css"
            } else if file.ends_with(".js") {
                "application/javascript"
            } else {
                "text/plain"
            };

            let response = Response::builder()
                .header("Content-Type", content_type)
                .body(axum::body::Body::from(content))
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(response)
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

// OAuth callback handler




// Serve posts as HTML for HTMX
async fn serve_posts_html() -> Html<String> {
    // Use absolute path to posts.json
    let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let posts_path = current_dir.join("posts.json");
    
    match std::fs::read_to_string(posts_path) {
        Ok(content) => match serde_json::from_str::<Vec<crate::markdown::Post>>(&content) {
            Ok(posts) => {
                let mut html = String::new();

                if posts.is_empty() {
                    html.push_str("<p class='no-posts'>No posts available yet.</p>");
                } else {
                    for post in posts {
                        let date = post.created_at.format("%B %d, %Y").to_string();

                        html.push_str(&format!(
                            r#"
<article class="post-card">
    <div class="post-header">
        <h3 class="post-title">
            <a href="/posts/{}" class="post-link">{}</a>
        </h3>
        <div class="post-meta">
            <span class="post-author">By {}</span>
            <span class="post-date">{}</span>
        </div>
    </div>
    <div class="post-actions">
        <a href="/posts/{}" class="btn btn-primary">Read More</a>
    </div>
</article>
                                "#,
                            post.slug, post.title, post.author, date, post.slug
                        ));
                    }
                }

                Html(html)
            }
            Err(_) => Html("<p class='no-posts'>Error loading posts.</p>".to_string()),
        },
        Err(_) => Html("<p class='no-posts'>Error loading posts.</p>".to_string()),
    }
}

// Serve favicon
async fn serve_favicon() -> Result<Response, StatusCode> {
    // Create a simple favicon response or return 204 No Content
    let response = Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(axum::body::Body::empty())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(response)
}
