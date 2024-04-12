use anyhow::Result;
use dotenv::dotenv;
use ngrok::prelude::*;
use std::env;
use std::net::SocketAddr;
use tokio::task;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use vc_issuer::{
    routes::router,
    server::{serve, AppState},
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::new("debug"))
        .init();

    let mut tun = ngrok::Session::builder()
        .authtoken_from_env()
        .connect()
        .await
        .unwrap()
        .http_endpoint()
        .listen()
        .await
        .unwrap();

    let base_url = tun.url().to_string();

    tracing::info!("App URL: {:?}", base_url);

    let seed = env::var("SEED").unwrap_or_else(|_| "seed123!".to_string());
    //let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    //let domain = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    let domain = base_url
        .replace("http://", "")
        .replace("https://", "")
        .replace(".ngrok.io", "");

    let app = AppState {
        base_url,
        domain,
        seed,
    };

    let ngrok_tunnel = task::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tun.forward_tcp(addr).await.unwrap();
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    let app = router(app).layer(TraceLayer::new_for_http());
    tracing::info!("Running {}", listener.local_addr()?);

    let server = task::spawn(async move {
        serve(listener, app).await.unwrap();
    });

    let _ = tokio::try_join!(ngrok_tunnel, server)?;
    Ok(())
}
