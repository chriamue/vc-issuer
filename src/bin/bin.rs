use std::env;

use anyhow::Result;
use dotenv::dotenv;
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
        .init();

    let seed = env::var("SEED").unwrap_or_else(|_| "seed123!".to_string());
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let domain = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    let app = AppState {
        base_url,
        domain,
        seed,
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    let app = router(app);
    println!("Running {}", listener.local_addr()?);

    serve(listener, app).await?;
    Ok(())
}
