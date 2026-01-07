# Serverless AI Agent on Civo (Wasm + Rust)

![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange)
![Wasm](https://img.shields.io/badge/WebAssembly-Spin-blue)
![Civo](https://img.shields.io/badge/Civo-Kubernetes-purple)

A demonstration of **"Post-Docker"** architecture. This project runs a Sentiment Analysis AI Agent compiled to **WebAssembly**, orchestrated by **SpinKube** on a Kubernetes cluster.

## 🚀 Why Wasm?
*   **Size:** The final binary is < 5MB.
*   **Startup:** < 5ms cold start.
*   **Security:** Sandboxed by default.

## 🛠️ Architecture
1.  **Rust Code:** Handles HTTP logic.
2.  **Spin SDK:** Offloads AI inference to the host node (Serverless AI).
3.  **Kubernetes:** Orchestrates the Wasm modules via the `spin-operator`.

## ⚡ Quick Start (Local)

### Prerequisites
*   [Rust](https://www.rust-lang.org/) & `wasm32-wasi` target.
*   [Spin CLI](https://developer.fermyon.com/spin/install).

### Build & Run
```bash
# Install Wasm target
rustup target add wasm32-wasi

# Build the project
spin build

# Run locally (Simulating Civo)
spin up --ai
```