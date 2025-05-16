use std::time::Duration;

use qdrant_client::prelude::*;
use qdrant_client::qdrant::{Distance, VectorParams, vectors_config};

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
        api_key: None,
        compression: None,
        check_compatibility: false,
    };

    let qdrant_client = Qdrant::new(q_config)?;

    let create_collection = CreateCollectionBuilder::new("documents").vector_config(
        VectorParamsBuilder::new(2048, qdrant_client::qdrant::Distance::Cosine).build(),
    );
    qdrant_client.create_collection(create_collection).await?;

    let documents = vec![
    Document {
        id: 1,
        text: "Rust is a systems programming language that is fast, memory-efficient, and type-safe.".to_string(),
    },
    Document {
        id: 2,
        text: "Python is a popular programming language for web development and data science.".to_string(),
    },
    Document {
        id: 3,
        text: "JavaScript is a versatile language commonly used for building interactive web interfaces.".to_string(),
    },
    Document {
        id: 4,
        text: "Go is known for its simplicity and efficiency in building scalable backend services.".to_string(),
    },
    Document {
        id: 5,
        text: "C++ is a powerful language often used in game engines, embedded systems, and performance-critical applications.".to_string(),
    },
    Document {
        id: 6,
        text: "TypeScript extends JavaScript by adding types, improving developer experience and code quality.".to_string(),
    },
    Document {
        id: 7,
        text: "Kotlin is a modern programming language designed to be fully interoperable with Java and improve Android development.".to_string(),
    },
];
    Ok(())
}

async fn generate_embedding_for_text(text: &str) -> Vec<f32> {
    let client = Client::new();

    let response = Client
        .post(OLLAMA_API_URL)
        .json(&serde_json::json!({
            "model": OLLAMA_MODEL,
            "input": text,
        }))
        .send()
        .await
        .expect("Failed to generate embeddings");

    dbg!(response.status());

    let embedding_data: OllamaEmbeddingResponse =
        response.json().await.expect("Failed to parse response");

    embedding_data.embeddings
}
