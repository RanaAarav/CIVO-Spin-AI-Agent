Serverless AI Inference on K3s with Wasm and Rust
=================================================

![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange)
![Wasm](https://img.shields.io/badge/WebAssembly-Spin-blue)
![Civo](https://img.shields.io/badge/Civo-Kubernetes-purple)

**The Problem:** Traditional AI containers (Python/Docker) are bloated, often exceeding 2GB, and suffer from cold start latency that kills agentic responsiveness.

**The Solution:** This repository demonstrates a high-performance alternative. By using Rust and the Spin Framework, we compile AI agents into WebAssembly (WASM) binaries. These are deployed to Civo's K3s clusters using the SpinKube operator, achieving really low latency.

Technology Stack
----------------

*   **Runtime:** WebAssembly (WASI-P1)
*   **Framework:** Spin SDK (Fermyon)
*   **Orchestration:** Civo K3s + SpinKube Operator
*   **Language:** Rust 1.8x

Key Features
------------

*   **Micro-Binaries:** 2.4MB total artifact size vs. 2GB+ Docker images.   
*   **Serverless AI:** Offloads LLM inference to the host capabilities using spin-sdk::llm.
*   **Cloud-Native:** Built for seamless deployment on Civo's optimized Kubernetes nodes.
    
Quick Start
-----------

1.  Install the Wasm target: ```bash rustup target add wasm32-wasip1```
2.  Build: ```bash spin build```
3.  Local Test: ```bash spin up --ai```
    
Structure
---------

*   /src: Rust source code for the inference agent.
*   spin.toml: Component manifest for the Wasm runtime.
*   deploy/: Kubernetes manifests for Civo deployment.
