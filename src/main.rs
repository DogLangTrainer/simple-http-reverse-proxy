use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode, Uri},
    response::Response,
    routing::get,
    Router,
};
use clap::Parser;
use reqwest::Client;
use std::net::SocketAddr;
use tracing::{error, info};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 3000, help = "Specify the port to listen on")]
    port: u16,

    #[arg(long, default_value = "/", help = "Specify a URL path prefix for the proxy route (e.g., 'proxy')")]
    prefix: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36")
        .build()
        .expect("Failed to build reqwest client");

    let clean_prefix = cli.prefix.trim_matches('/');
    
    let route_path = if clean_prefix.is_empty() {
        "/*target".to_string()
    } else {
        format!("/{clean_prefix}/*target")
    };

    info!("Proxy route set to: {}", route_path);
    let app = Router::new()
        .route(&route_path, get(proxy_handler))
        .with_state(client);

    let addr = SocketAddr::from(([0, 0, 0, 0], cli.port));
    info!("Reverse proxy server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn proxy_handler(
    Path(target): Path<String>,
    axum::extract::State(client): axum::extract::State<Client>,
    req: Request<Body>,
) -> Result<Response, StatusCode> {
    let target_url_str = if let Some(stripped) = target.strip_prefix('/') {
        stripped
    } else {
        &target
    };

    let target_uri: Uri = match target_url_str.parse() {
        Ok(uri) => uri,
        Err(_) => {
            error!("Invalid target URL: {}", target_url_str);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    info!("Proxying request to: {}", target_uri);

    let mut headers = req.headers().clone();
    headers.remove(http::header::HOST);

    let upstream_res = match client
        .get(target_uri.to_string())
        .headers(headers)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => {
            error!("Upstream request to {} failed: {}", target_uri, e);
            return Err(StatusCode::BAD_GATEWAY);
        }
    };
    
    info!("Received response {} from {}", upstream_res.status(), target_uri);

    let mut response_builder = Response::builder().status(upstream_res.status());
    *response_builder.headers_mut().unwrap() = upstream_res.headers().clone();

    let stream = upstream_res.bytes_stream();
    let body = Body::from_stream(stream);

    response_builder
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}