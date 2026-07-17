# gouda-core-rs

[![Rust](https://github.com/gonicus/gouda-core-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/gonicus/gouda-core-rs/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Core Rust library for building [GOuda API](https://github.com/gonicus/gouda-proto) chat clients.

## Overview

GOuda Core provides an async abstraction layer over the GOuda API, enabling developers to implement
chat clients in Rust. It defines a `Client` trait that implementations must fulfill to handle
GOuda protocol requests, along with a `Runner` that manages the full request-response lifecycle
including input processing, execution, and output.

For a complete example of a GOuda client using local sockets, see [gouda-matrix](https://github.com/gonicus/gouda-matrix).

## Crates

This workspace contains the following crates:

| Crate | Description |
|-------|-------------|
| [gouda_core](gouda_core/) | Core library providing the `Client` trait and `Runner` for building GOuda chat clients. |
| [gouda_proto](gouda_proto/) | Compiled Protocol Buffers for the GOuda API, including the chat protocol. |
| [gouda_sandbox](gouda_sandbox/) | A desktop sandbox application for testing and debugging GOuda clients. |

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable)
- [Protoc](https://github.com/protocolbuffers/protobuf) (for building protobuf definitions)
- [just](https://github.com/casey/just) (optional, for running commands via the justfile)

### Building

```bash
# Clone the repository including submodules
git clone --recursive https://github.com/gonicus/gouda-core-rs.git
cd gouda-core-rs
cargo build
```

### Running Tests

```bash
just test
```

### Code Quality Checks

```bash
# Run all checks (clippy, fmt, tests, unused deps, typos)
just check

# Format code
just fmt
```

## Usage

### Implementing a Client

To build a GOuda chat client, implement the `Client` trait:

```rust
use gouda_core::{Client, RequestContext, Result};
use gouda_proto::chat::*;
use async_trait::async_trait;
use std::any::Any;

struct MyClient;

#[async_trait]
impl Client for MyClient {
    async fn initialize(&self, ctx: RequestContext, request: InitializationRequest) -> Result<StatusUpdate> {
        // Handle initialization
        todo!()
    }

    async fn get_login_flows(&self, ctx: RequestContext) -> Result<LoginFlowsResponse> {
        // Return available login flows
        todo!()
    }

    // ... implement other required methods

    fn as_any(&self) -> &dyn Any {
        self
    }
}
```

### Running the Runner

The `Runner` manages the full lifecycle: reading requests from an input source, executing them
via your client, and writing responses back.

```rust
use gouda_core::Runner;
use std::sync::Arc;

let client: Arc<dyn Client> = Arc::new(MyClient);
let runner = Runner::new(client, reader, writer);
runner.run().await?;
```

### Sandbox Application

A graphical sandbox application is included for testing and debugging GOuda clients. It provides
a UI for sending requests and inspecting responses over two local sockets.

```bash
cargo run --bin gouda_sandbox <request_socket> <response_socket> [--config <path>]
```

See [gouda_sandbox/README.md](gouda_sandbox/README.md) for details.
