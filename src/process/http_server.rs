use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tracing::{info, warn};

use crate::{cli::HttpServeOpts, Process};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

impl Process for HttpServeOpts {
    async fn process(&self) -> anyhow::Result<()> {
        let path = self.dir.clone();
        let port = self.port;

        process_http_server(path, port).await?;

        Ok(())
    }
}

pub async fn process_http_server(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path };
    // axum router
    let router = Router::new()
        .route("/", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Requesting file: {:?}", p);

    if !p.exists() {
        (StatusCode::NOT_FOUND, "File not found".to_string())
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
