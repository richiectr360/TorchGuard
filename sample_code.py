def train_model(model, dataloader, optimizer, epochs=10):
    for epoch in range(epochs):
        model.train()
        for batch in dataloader:
            optimizer.zero_grad()
            inputs, labels = batch
            inputs = inputs.cuda()  # Device management
            outputs = model(inputs.float())  # Data type conversion
            loss = criterion(outputs, labels)
            loss.backward()
            optimizer.step()

def evaluate_model(model, val_loader):
    model.eval()
    total_loss = 0
    with torch.no_grad():  # Memory optimization
        for batch in val_loader:
            inputs, labels = batch
            outputs = model(inputs)
            loss = criterion(outputs, labels)
            total_loss += loss.item()
    return total_loss / len(val_loader)

def data_preparation():
    transform = transforms.Compose([
        transforms.ToTensor(),
        transforms.Normalize((0.5,), (0.5,))
    ])
    dataset = datasets.MNIST(root='./data', train=True, transform=transform)
    dataloader = DataLoader(dataset, batch_size=32, num_workers=4)  # Efficient data loading
    return dataloader
