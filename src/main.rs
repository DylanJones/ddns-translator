use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/update", get(update_handler).layer(middleware::from_fn(basic_auth)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>DDNS Translator</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            background-color: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        h1 {
            color: #333;
        }
        p {
            color: #666;
            line-height: 1.6;
        }
        .endpoint {
            background-color: #f9f9f9;
            padding: 10px;
            border-left: 4px solid #4CAF50;
            margin: 10px 0;
        }
        code {
            background-color: #e8e8e8;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: monospace;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>DDNS Translator</h1>
        <p>A small utility to translate NoIP-style Dynamic DNS update requests to RFC2136 DNS update requests.</p>
        
        <h2>Available Endpoints</h2>
        <div class="endpoint">
            <strong>GET /</strong><br>
            This information page
        </div>
        <div class="endpoint">
            <strong>GET /update</strong><br>
            Dynamic DNS update endpoint (requires HTTP Basic authentication)
        </div>
        
        <h2>Status</h2>
        <p>Server is running and ready to process requests.</p>
        
        <h2>Usage</h2>
        <p>To update your dynamic DNS, send a GET request to <code>/update</code> with HTTP Basic authentication credentials.</p>
    </div>
</body>
</html>"#,
    )
}

async fn update_handler() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>DDNS Update - Authenticated</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            background-color: white;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        h1 {
            color: #4CAF50;
        }
        p {
            color: #666;
            line-height: 1.6;
        }
        .success {
            background-color: #d4edda;
            border: 1px solid #c3e6cb;
            color: #155724;
            padding: 15px;
            border-radius: 4px;
            margin: 20px 0;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>✓ Authentication Successful</h1>
        <div class="success">
            <strong>Status:</strong> Authenticated successfully
        </div>
        <p>This is a placeholder page for the Dynamic DNS update endpoint.</p>
        <p>Future functionality will include:</p>
        <ul>
            <li>IP address detection from request</li>
            <li>RFC2136 DNS update translation</li>
            <li>Response formatting compatible with NoIP clients</li>
        </ul>
    </div>
</body>
</html>"#,
    )
}

async fn basic_auth(req: Request, next: Next) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        if auth_header.starts_with("Basic ") {
            let credentials = &auth_header[6..];
            if let Ok(decoded) = general_purpose::STANDARD.decode(credentials) {
                if let Ok(credential_str) = String::from_utf8(decoded) {
                    // For demonstration purposes, accept username:password as "admin:password"
                    // In production, this should be configurable and use secure password hashing
                    if credential_str == "admin:password" {
                        return Ok(next.run(req).await);
                    }
                }
            }
        }
    }

    // Return 401 Unauthorized with WWW-Authenticate header
    let mut response = (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    response.headers_mut().insert(
        header::WWW_AUTHENTICATE,
        header::HeaderValue::from_static(r#"Basic realm="DDNS Translator""#),
    );
    Err(response)
}
