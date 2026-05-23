# grpc

Tonic-based gRPC example with a generated protobuf service, server binary, and client binary.

## What it shows

- Protobuf-driven service definition in `proto/hello.proto`
- Code generation through `build.rs`
- Separate client and server entry points

## Run

Start the server:

```bash
cargo run --bin server
```

Run the client in another terminal:

```bash
cargo run --bin client
```