use qdrant_client::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    id: u64,
    text: String,
}

#[derive(Debug, Deserialize)]
struct OllamaEmbeddingResponse {
    embeddings: Vec<Vec<f32>>,
}

const OLLAMA_API_URL: &str = "http://localhost:11434";
const OLLAMA_MODEL: &str = "llama3:2.1b";

#[tokio::main]
async fn main() -> Result<(), QdrantClientError> {
    let q_config = QdrantClientConfig {
        uri: "http://localhost:6334".to_string(),
        timeout: Some(Duration::from_secs(10)),
        api_key: None,
        ..Default::default()
    };

    let client = QdrantClient::new(Some(q_config))?;

    let collection_name = "documents";

    let create = CreateCollection {
        collection_name: collection_name.to_string(),
        vectors_config: Some(VectorsConfig {
            config: Some(vectors_config::Config::Params(VectorParams {
                size: 2048,
                distance: Distance::Cosine.into(),
                ..Default::default()
            })),
        }),
        ..Default::default()
    };

    client.create_collection(&create).await?;

    println!("✅ Collection created!");

    Ok(())
}
