Serverless AI Inference via Wasm on Civo K3s
============================================

![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange)
![Wasm](https://img.shields.io/badge/WebAssembly-Spin-blue)
![Civo](https://img.shields.io/badge/Civo-Kubernetes-purple)

This repository provides a reference architecture for deploying AI agents using WebAssembly (Wasm) on Civo's Kubernetes service.

Standard Docker-based AI deployments often face significant cold-start latency and heavy resource overhead. By using the Spin framework and Rust, this project compiles inference logic into a Wasm binary under 5MB. This approach enables sub-millisecond startup times and higher pod density per node compared to traditional containerized Python environments.

Technical Stack
---------------
*   Runtime: WebAssembly (WASI-P1)
*   Framework: Spin SDK 3.0
*   Infrastructure: Civo K3s with SpinKube Operator
*   Language: Rust 1.84+

Features
--------
*   Sub-millisecond cold starts for agentic responsiveness.
*   Minimal attack surface via Wasm sandboxing.
*   Native host-facilitated LLM inference using ```spin-sdk::llm```
    
Start
-----
1.  Add the Wasm target: ```rustup target add wasm32-wasip1```
2.  Compile: ```spin build```
3.  Local Test: ```spin up --ai``` (Requires local GGUF model)
    
Structure
---------
*   /src: Rust source code for the inference agent.
*   spin.toml: Component manifest for the Wasm runtime.
*   deploy/: Kubernetes manifests for Civo deployment.
