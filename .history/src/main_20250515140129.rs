use qdrant_client::prelude::*;
use serde::{Deserialize, Serialize};

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
async fn main() -> Result<(), QdrantError> {
    let q_config = QdrantConfig {
        uri: String::from("http://localhost:6334"),
        timeout: Duration::from_secs(10),
        api_key = None,
        compression: None,
        check_compatibility: false,
    };

    let qdrant_client = Qdrant::new(q_config)?;
    
    let create_collection = CreateCollectionBuilder::new(
        "documents"
    ).vector_config(
        VectorParamsBuilder::new(2048, qdrant_client::qdrant::Distance::Cosine)
        .quantization_config((ScalarQuantizationBuilder::default()
    );
}
