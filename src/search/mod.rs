use anyhow::Result;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CodeSearch {
    client: Client,
    ollama_url: String,
    qdrant_url: String,
    collection: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub score: f32,
    pub payload: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResponse {
    result: Vec<SearchResult>,
}

impl CodeSearch {
    pub fn new(ollama_url: &str, qdrant_url: &str, collection: &str) -> Self {
        CodeSearch {
            client: Client::new(),
            ollama_url: ollama_url.to_string(),
            qdrant_url: qdrant_url.to_string(),
            collection: collection.to_string(),
        }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        // Generate embedding with temperature 0 for consistency
        let embedding = self.generate_embedding(query).await?;
        
        // Search with filters and better scoring
        let response = self.client
            .post(format!("{}/collections/{}/points/search", self.qdrant_url, self.collection))
            .json(&serde_json::json!({
                "vector": embedding,
                "limit": 5,
                "with_payload": true,
                "with_vectors": true,
                "score_threshold": 0.3,
                "params": {
                    "hnsw_ef": 128
                }
            }))
            .send()
            .await?
            .json::<SearchResponse>()
            .await?;

        // Post-process results
        let mut results = response.result;
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        // Remove duplicates and very similar results
        results.dedup_by(|a, b| {
            let sim = self.compute_similarity(
                a.payload.get("content").and_then(|v| v.as_str()).unwrap_or(""),
                b.payload.get("content").and_then(|v| v.as_str()).unwrap_or("")
            );
            sim > 0.9
        });

        Ok(results)
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        #[derive(Deserialize)]
        struct EmbeddingResponse {
            embedding: Vec<f32>,
        }

        let response = self.client
            .post(&self.ollama_url)
            .json(&serde_json::json!({
                "model": "llama2",
                "prompt": text.trim(),
                "options": {
                    "temperature": 0.0,
                    "num_ctx": 2048
                }
            }))
            .send()
            .await?
            .json::<EmbeddingResponse>()
            .await?;

        Ok(response.embedding)
    }

    fn compute_similarity(&self, text1: &str, text2: &str) -> f32 {
        // Simple Jaccard similarity for quick comparison
        let words1: std::collections::HashSet<_> = text1.split_whitespace().collect();
        let words2: std::collections::HashSet<_> = text2.split_whitespace().collect();
        
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        intersection as f32 / union as f32
    }
}

#[cfg(test)]
mod tests;
