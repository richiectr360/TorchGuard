# ML Code Assistant

## The Problem

When developing deep learning models, it's easy to overlook critical performance optimizations and best practices. Common issues include:

- Inefficient GPU memory usage that could lead to OOM errors
- Unnecessary gradient computations during inference
- Memory leaks from not clearing gradients
- Suboptimal batch sizes affecting training speed
- Missing performance improvements from learning rate scheduling

This tool automatically identifies these issues in your PyTorch code and suggests improvements, helping you write more efficient and robust deep learning applications.

## Features

✨ **Automatic Code Analysis**: Scans your code for common performance pitfalls
🚀 **One-Click Fixes**: Apply suggested optimizations with a single click
🎯 **Best Practices**: Learn better coding patterns through actionable feedback
💡 **Educational**: Understand why each optimization matters

## Project Structure

- `/src` - Rust backend for code analysis
- `/ml-code-assistant` - Next.js frontend

## Prerequisites

- Rust (latest stable)
- Node.js (v18 or later)
- npm

## Getting Started

### Backend Setup

1. Navigate to the root directory:
```bash
cd rust-llm-qdrant
```

2. Run the Rust backend:
```bash
cargo run
```

The backend will start on http://localhost:3003

### Frontend Setup

1. Navigate to the frontend directory:
```bash
cd ml-code-assistant
```

2. Install dependencies:
```bash
npm install
```

3. Start the development server:
```bash
npm run dev
```

The frontend will be available at http://localhost:3001

## Usage

1. Open http://localhost:3001 in your browser
2. Paste your PyTorch code into the editor
3. Click "Analyze Code"
4. Review the suggestions in the right panel
5. Apply fixes by clicking "Apply Fix" on any suggestion

## Example Code

Here's a sample code snippet to test the analyzer:

```python
import torch
import torch.nn as nn

# Define a simple CNN model
class CNN(nn.Module):
    def __init__(self):
        super(CNN, self).__init__()
        self.conv1 = nn.Conv2d(3, 16, 3)
        self.pool = nn.MaxPool2d(2, 2)
        self.fc1 = nn.Linear(16 * 14 * 14, 10)

    def forward(self, x):
        x = self.pool(torch.relu(self.conv1(x)))
        x = x.view(-1, 16 * 14 * 14)
        return self.fc1(x)

# Training setup
model = CNN()
model.cuda()  # Non-device-agnostic GPU usage
criterion = nn.CrossEntropyLoss()
optimizer = torch.optim.Adam(model.parameters(), lr=0.001)
batch_size = 24  # Not a power of 2

# Training loop
for epoch in range(10):
    for data, target in train_loader:
        data = data.cuda()  # Non-device-agnostic GPU usage
        target = target.cuda()
        
        # Missing optimizer.zero_grad()
        output = model(data)
        loss = criterion(output, target)
        loss.backward()
        optimizer.step()

# Inference
for data, target in test_data:
    # Missing model.eval() and torch.no_grad()
    output = model(data.cuda())
    predictions = output.max(1)[1]
```

## Roadmap

🚧 **Coming Soon**

### Infrastructure
- CI/CD pipeline with AWS infrastructure
- Automated testing and deployment
- Scalable backend architecture

### Advanced Analysis
- Distributed training optimization suggestions
- Multi-GPU data parallelism best practices
- Mixed precision training recommendations
- Gradient accumulation for large models
- Memory-efficient attention patterns
- Transformer architecture optimizations

### Performance Monitoring
- Real-time memory usage tracking
- GPU utilization analysis
- Training throughput optimization
- Bottleneck identification
- Hardware-specific tuning suggestions

## Contributing

Feel free to submit issues and enhancement requests!
