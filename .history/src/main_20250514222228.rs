use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    id: u64,
    text: String,
}

struct OllamaEmbeddingResponse {
    embeddings: Vec<Vec<f32>>,
}
