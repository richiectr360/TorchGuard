use anyhow::Result;
use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_analyze_endpoint() -> Result<()> {
    let client = Client::new();
    let code = r#"
def train_model(model, data):
    model.cuda()
    outputs = model(data.cuda())
    loss = criterion(outputs)
    loss.backward()
    optimizer.step()
"#;

    let response = client
        .post("http://localhost:3001/analyze")
        .json(&json!({
            "code": code
        }))
        .send()
        .await?;

    assert!(response.status().is_success());
    
    let findings: Vec<serde_json::Value> = response.json().await?;
    assert!(!findings.is_empty(), "Should find at least one issue");
    
    // Check if it found the cuda() usage
    let has_cuda_warning = findings.iter().any(|f| {
        f["category"].as_str().unwrap_or("") == "GPU Usage" &&
        f["message"].as_str().unwrap_or("").contains("device-agnostic")
    });
    assert!(has_cuda_warning, "Should detect cuda() usage");

    Ok(())
}

#[tokio::test]
async fn test_search_endpoint() -> Result<()> {
    let client = Client::new();
    let query = r#"
def predict(model, data):
    model.eval()
    with torch.no_grad():
        return model(data)
"#;

    let response = client
        .post("http://localhost:3001/search")
        .json(&json!({
            "query": query
        }))
        .send()
        .await?;

    assert!(response.status().is_success());
    
    let results: Vec<serde_json::Value> = response.json().await?;
    assert!(!results.is_empty(), "Should find at least one similar code");

    Ok(())
}

#[tokio::test]
async fn test_health_endpoint() -> Result<()> {
    let client = Client::new();
    let response = client
        .get("http://localhost:3001/health")
        .send()
        .await?;

    assert!(response.status().is_success());
    assert_eq!(response.text().await?, "OK");

    Ok(())
}
