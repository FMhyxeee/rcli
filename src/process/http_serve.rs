use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use askama_axum::Template;
use axum::{extract::State, response::Html, routing::get, Router};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);

    let state = HttpServeState { path: path.clone() };
    // axum router
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(path))
        .route("/", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(State(state): State<Arc<HttpServeState>>) -> Html<String> {
    let path = &state.path;
    let file_list = match path.read_dir() {
        Ok(files) => files
            .filter_map(|f| f.ok())
            .filter_map(|f| f.file_name().into_string().ok())
            .collect(),
        Err(e) => {
            warn!("Failed to read directory: {:?}", e);
            Vec::new()
        }
    };

    let index = IndexTemplate { file_list };

    match index.render() {
        Ok(content) => Html(content),
        Err(e) => {
            warn!("Failed to render template: {:?}", e);
            Html("Failed to render template".to_string())
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    file_list: Vec<String>,
}
