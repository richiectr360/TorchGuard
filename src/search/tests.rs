#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_creation() {
        let search = CodeSearch::new(
            "http://localhost:11434",
            "http://localhost:6333",
            "test_collection"
        );
        assert_eq!(search.collection, "test_collection");
    }

    #[tokio::test]
    async fn test_search_similar_code() -> Result<()> {
        let search = CodeSearch::new(
            "http://localhost:11434",
            "http://localhost:6333",
            "code_snippets"
        );

        let query = r#"
def predict(model, data):
    model.eval()
    with torch.no_grad():
        return model(data)
"#;

        let results = search.search(query).await?;
        assert!(!results.is_empty(), "Should find at least one similar code");
        
        // Check result structure
        let first_result = &results[0];
        assert!(first_result.score > 0.0, "Score should be positive");
        assert!(first_result.payload.contains_key("content"), "Should have content");
        assert!(first_result.payload.contains_key("language"), "Should have language");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_search_no_results() -> Result<()> {
        let search = CodeSearch::new(
            "http://localhost:11434",
            "http://localhost:6333",
            "code_snippets"
        );

        let query = "def this_does_not_exist(): pass";
        let results = search.search(query).await?;
        assert!(results.is_empty(), "Should not find any results");
        
        Ok(())
    }
}
