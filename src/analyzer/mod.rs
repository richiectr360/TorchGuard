use anyhow::Result;
use serde::Serialize;
use tree_sitter::{Parser, Query, QueryCursor};
use std::collections::HashMap;

pub struct CodeAnalyzer {
    parser: Parser,
    query_cache: HashMap<String, Query>,
}

#[derive(Debug, Serialize)]
pub struct Fix {
    pub description: String,
    pub code: String,
    pub range: Range,
}

#[derive(Debug, Serialize)]
pub struct Range {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug, Serialize)]
pub struct Finding {
    pub category: String,
    pub message: String,
    pub line: i32,
    pub severity: String,
    pub fix: Option<Fix>,
}

impl CodeAnalyzer {
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_python::language())?;
        
        Ok(CodeAnalyzer {
            parser,
            query_cache: HashMap::new(),
        })
    }

    pub fn analyze(&mut self, code: &str) -> Result<Vec<Finding>> {
        println!("Analyzing code:\n{}", code);
        let tree = self.parser.parse(code, None).ok_or_else(|| anyhow::anyhow!("Failed to parse code"))?;
        let mut findings = Vec::new();

        // Advanced ML patterns query
        let ml_query = self.get_or_create_query(r#"
            ; GPU and Memory Operations
            (call function: (attribute object: (_) @obj attribute: (identifier) @attr) @cuda_call)
            (with_statement body: (block . (expression_statement (call function: (_) @func))))
            ; Function definitions
            (function_definition name: (identifier) @func_name parameters: (parameters) @params body: (block) @body)
            ; Method calls
            (call function: (attribute object: (_) @method_obj attribute: (identifier) @method_name) @method_call)
            ; Variable assignments
            (assignment left: (_) @assign_left right: (_) @assign_right)
            ; For loops
            (for_statement body: (block) @for_body)
            ; Comments
            (comment) @comment
            ; Training operations
            (call function: (attribute object: (_) @train_obj attribute: (identifier) @train_method) @train_call)
        "#)?;

        let mut query_cursor = QueryCursor::new();
        let matches = query_cursor.matches(&ml_query, tree.root_node(), code.as_bytes());

        for match_ in matches {
            for capture in match_.captures {
                let capture_text = capture.node.utf8_text(code.as_bytes())?;
                let line_number = capture.node.start_position().row as i32;
                
                // Device placement checks
                if capture_text.contains(".cuda()") {
                    findings.push(Finding {
                        category: String::from("GPU Usage"),
                        message: String::from("Consider using device-agnostic code with .to(device)"),
                        line: line_number + 1,
                        severity: String::from("Warning"),
                        fix: Some(Fix {
                            description: String::from("Use device-agnostic code"),
                            code: String::from("device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')\n# Use device-agnostic code\nmodel = model.to(device)"),
                            range: Range {
                                start: line_number,
                                end: line_number + 1,
                            },
                        }),
                    });
                }

                // Memory management
                if capture_text == "backward" && !code.contains("torch.no_grad()") {
                    findings.push(Finding {
                        category: String::from("Memory Usage"),
                        message: String::from("Use torch.no_grad() for inference"),
                        line: line_number + 1,
                        severity: String::from("Warning"),
                        fix: Some(Fix {
                            description: String::from("Add torch.no_grad() context"),
                            code: String::from("with torch.no_grad():\n    output = model(data)"),
                            range: Range {
                                start: line_number,
                                end: line_number + 1,
                            },
                        }),
                    });
                }

                // Gradient accumulation
                if capture_text == "backward" && !code.contains(".zero_grad()") {
                    findings.push(Finding {
                        category: "Training".to_string(),
                        message: "Consider calling optimizer.zero_grad() before backward pass".to_string(),
                        line: line_number + 1,
                        severity: "Warning".to_string(),
                        fix: Some(Fix {
                            description: "Add optimizer.zero_grad()".to_string(),
                            code: "optimizer.zero_grad()  # Clear gradients before backward pass\nloss.backward()".to_string(),
                            range: Range {
                                start: line_number,
                                end: line_number + 1,
                            },
                        }),
                    });
                }

                // DataLoader optimization
                if capture_text.contains("DataLoader") && !code.contains("num_workers") {
                    findings.push(Finding {
                        category: "Performance".into(),
                        message: "Set num_workers in DataLoader for faster data loading".into(),
                        line: line_number + 1,
                        severity: "Info".into(),
                        fix: None,
                    });
                }

                // Mixed precision training
                if capture_text == "backward" && !code.contains("amp") && !code.contains("autocast") {
                    findings.push(Finding {
                        category: "Performance".to_string(),
                        message: "Consider using mixed precision training with torch.cuda.amp".to_string(),
                        line: line_number + 1,
                        severity: "Info".to_string(),
                        fix: Some(Fix {
                            description: "Add mixed precision training".to_string(),
                            code: "scaler = torch.cuda.amp.GradScaler()\nwith torch.cuda.amp.autocast():\n    output = model(data)\n    loss = criterion(output, targets)\nscaler.scale(loss).backward()\nscaler.step(optimizer)\nscaler.update()".to_string(),
                            range: Range {
                                start: line_number,
                                end: line_number + 1,
                            },
                        }),
                    });
                }

                // Batch size power of 2
                if capture_text.contains("batch_size") {
                    let batch_size_pattern = regex::Regex::new(r"batch_size\s*=\s*(\d+)").unwrap();
                    if let Some(caps) = batch_size_pattern.captures(code) {
                        if let Some(size_str) = caps.get(1) {
                            if let Ok(size) = size_str.as_str().parse::<i32>() {
                                if (size & (size - 1)) != 0 {  // Check if not power of 2
                                    findings.push(Finding {
                                        category: String::from("Performance"),
                                        message: String::from("Consider using a power of 2 for batch size for optimal GPU utilization"),
                                        line: line_number + 1,
                                        severity: String::from("Info"),
                                        fix: None,
                                    });
                                }
                            }
                        }
                    }
                }

                // Model evaluation mode
                if capture_text == "forward" && !code.contains(".eval()") && code.contains("test_data") {
                    findings.push(Finding {
                        category: String::from("Model State"),
                        message: String::from("Set model.eval() for inference or validation"),
                        line: line_number + 1,
                        severity: String::from("Warning"),
                        fix: Some(Fix {
                            description: String::from("Add model.eval() before inference"),
                            code: String::from("model.eval()  # Set model to evaluation mode"),
                            range: Range {
                                start: line_number,
                                end: line_number + 1,
                            },
                        }),
                    });
                }

                // Check for training operations
                let capture_text = capture.node.utf8_text(code.as_bytes())?;
                let line_number = capture.node.start_position().row as i32;

                // Check for GPU operations
                if capture_text.contains(".cuda()") {
                    findings.push(Finding {
                        category: String::from("GPU Usage"),
                        message: String::from("Consider using device-agnostic code with .to(device)"),
                        line: line_number + 1,
                        severity: String::from("Warning"),
                        fix: Some(Fix {
                            description: String::from("Use device-agnostic code"),
                            code: String::from("device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')\n# Use device-agnostic code\nmodel = model.to(device)"),
                            range: Range {
                                start: line_number,
                                end: line_number + 1,
                            },
                        }),
                    });
                }

                // Check for backward operations
                if capture_text == "backward" && !code.contains(".zero_grad()") {
                    findings.push(Finding {
                        category: String::from("Training"),
                        message: String::from("Call optimizer.zero_grad() before backward pass"),
                        line: line_number + 1,
                        severity: String::from("Warning"),
                        fix: Some(Fix {
                            description: String::from("Add optimizer.zero_grad()"),
                            code: String::from("optimizer.zero_grad()  # Clear gradients before backward pass"),
                            range: Range {
                                start: line_number,
                                end: line_number + 1,
                            },
                        }),
                    });
                }

                // Check for model evaluation
                if capture_text == "forward" && !code.contains(".eval()") && code.contains("test_data") {
                    findings.push(Finding {
                        category: String::from("Model State"),
                        message: String::from("Set model.eval() for inference or validation"),
                        line: line_number + 1,
                        severity: String::from("Warning"),
                        fix: Some(Fix {
                            description: String::from("Add model.eval() before inference"),
                            code: String::from("model.eval()  # Set model to evaluation mode"),
                            range: Range {
                                start: line_number,
                                end: line_number + 1,
                            },
                        }),
                    });
                }

                // Gradient clipping
                if capture_text == "backward" && !code.contains("clip_grad") {
                    findings.push(Finding {
                        category: String::from("Training Stability"),
                        message: String::from("Consider using gradient clipping for training stability"),
                        line: line_number + 1,
                        severity: String::from("Info"),
                        fix: None,
                    });
                }

                // Learning rate scheduler
                if capture_text.contains("optimizer") && !code.contains("scheduler") {
                    findings.push(Finding {
                        category: String::from("Training"),
                        message: String::from("Consider using a learning rate scheduler for better convergence"),
                        line: line_number + 1,
                        severity: String::from("Info"),
                        fix: Some(Fix {
                            description: String::from("Add learning rate scheduler"),
                            code: String::from("scheduler = torch.optim.lr_scheduler.ReduceLROnPlateau(optimizer, 'min')\nscheduler.step(val_loss)  # Update learning rate based on validation loss"),
                            range: Range {
                                start: line_number,
                                end: line_number + 1,
                            },
                        }),
                    });
                }
            }
        }

        println!("Found {} issues", findings.len());
        Ok(findings)
    }

    fn get_or_create_query(&mut self, query: &str) -> Result<&Query> {
        if !self.query_cache.contains_key(query) {
            let new_query = Query::new(tree_sitter_python::language(), query)?;
            self.query_cache.insert(query.to_string(), new_query);
        }
        Ok(self.query_cache.get(query).unwrap())
    }
}

#[cfg(test)]
mod tests;
