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


async 


