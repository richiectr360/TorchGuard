mod analyzer;
mod search;
mod api;

use colored::*;
use tracing_subscriber;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("{}", "ML Code Assistant".bright_green().bold());
    println!("{}", "Starting web server...".yellow());

    // Start web server
    let app = api::create_api().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await?;
    println!("{}", "✓ Server running at http://localhost:3003".green());
    println!("{}", "API Endpoints:".bright_blue());
    println!("  POST /analyze - Analyze code for optimization opportunities");
    println!("  POST /search  - Search for similar code patterns");
    println!("  GET  /health - Health check endpoint");

    axum::serve(listener, app).await?;

    Ok(())
}

// Rest of the code remains the same
