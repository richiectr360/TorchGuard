#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() -> Result<()> {
        let analyzer = CodeAnalyzer::new()?;
        assert!(analyzer.query_cache.is_empty());
        Ok(())
    }

    #[test]
    fn test_cuda_detection() -> Result<()> {
        let mut analyzer = CodeAnalyzer::new()?;
        let code = r#"
def process(model, data):
    model.cuda()
    return model(data.cuda())
"#;
        let findings = analyzer.analyze(code)?;
        
        assert_eq!(findings.len(), 2, "Should detect two cuda() calls");
        assert!(findings.iter().all(|f| f.category == "GPU Usage"));
        Ok(())
    }

    #[test]
    fn test_backward_detection() -> Result<()> {
        let mut analyzer = CodeAnalyzer::new()?;
        let code = r#"
def train_step(model, data):
    loss = model(data)
    loss.backward()
"#;
        let findings = analyzer.analyze(code)?;
        
        assert!(findings.iter().any(|f| 
            f.category == "Memory Usage" && 
            f.message.contains("torch.no_grad()")
        ));
        Ok(())
    }

    #[test]
    fn test_clean_code() -> Result<()> {
        let mut analyzer = CodeAnalyzer::new()?;
        let code = r#"
def predict(model, data, device):
    model.to(device)
    with torch.no_grad():
        return model(data.to(device))
"#;
        let findings = analyzer.analyze(code)?;
        assert!(findings.is_empty(), "Should not find issues in clean code");
        Ok(())
    }
}
