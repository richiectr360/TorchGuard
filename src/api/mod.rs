use axum::{
    routing::{get, post},
    Json, Router,
    extract::State,
};
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{analyzer::CodeAnalyzer, search::CodeSearch};

pub struct AppState {
    analyzer: Mutex<CodeAnalyzer>,
    search: CodeSearch,
}

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    code: String,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    findings: Vec<crate::analyzer::Finding>,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    query: String,
}

pub async fn create_api() -> Router {
    let analyzer = CodeAnalyzer::new().expect("Failed to create analyzer");
    let search = CodeSearch::new(
        "http://localhost:11434",
        "http://localhost:6333",
        "code_snippets",
    );

    let state = Arc::new(AppState {
        analyzer: Mutex::new(analyzer),
        search,
    });

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    Router::new()
        .route("/analyze", post(analyze_handler))
        .route("/search", post(search_handler))
        .route("/health", get(health_handler))
        .layer(cors)
        .with_state(state)
}

async fn analyze_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<AnalyzeRequest>,
) -> Json<AnalyzeResponse> {
    println!("Received code to analyze: {}\n", request.code);
    let mut analyzer = state.analyzer.lock().await;
    match analyzer.analyze(&request.code) {
        Ok(findings) => {
            println!("Analysis successful. Found {} issues.", findings.len());
            Json(AnalyzeResponse { findings })
        }
        Err(e) => {
            eprintln!("Error analyzing code: {}", e);
            Json(AnalyzeResponse { findings: vec![] })
        }
    }
}

async fn search_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SearchRequest>,
) -> Json<Vec<crate::search::SearchResult>> {
    let results = state.search.search(&request.query)
        .await
        .unwrap_or_default();
    
    Json(results)
}

async fn health_handler() -> &'static str {
    "OK"
}
