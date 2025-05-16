use serde::{Deserialize, Serialize};

struct Document {
    id: u64,
    text: String,
}

struct OllamaEmbeddingResponse {
    embeddings: Vec<Vec<f32>>,
}
