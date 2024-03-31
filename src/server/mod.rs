use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    pub base_url: String,
    pub seed: String,
}

pub async fn serve(listener: TcpListener, app: Router) -> Result<()> {
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| e.into())
}
