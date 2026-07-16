# gouda_sandbox

A desktop sandbox application for testing and debugging GOuda clients.

## Overview

This sandbox app provides a graphical interface for interacting with GOuda clients via local sockets. It enables developers to:

- Send GOuda requests and inspect responses in real time
- Test client functionality without a full application frontend
- Debug protocol-level issues

The app communicates with a GOuda client through two dedicated local sockets — one for sending requests and another for receiving responses.

## Usage

Build and run the sandbox app:

```bash
cargo run <request_socket> <response_socket> [--config <path>]
```

### Arguments

| Argument | Description |
|---|---|
| `request_socket` | Path to the local socket for sending requests to the GOuda client |
| `response_socket` | Path to the local socket for receiving responses and events from the GOuda client |
| `--config` | Path to the configuration file (default: `config.json` in the project directory) |

### Example

```bash
cargo run /tmp/request-socket /tmp/response-socket
```

The application will now start local socket servers at the specified paths, which your GOuda client
can connect to.
