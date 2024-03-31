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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = AppState {
        base_url: "http://localhost:3000".to_string(),
        seed: "seed123!".to_string(),
    };

    let app = router(app);
    println!("Running {}", listener.local_addr()?);

    serve(listener, app).await?;
    Ok(())
}
