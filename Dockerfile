FROM rust:1.55 as builder

RUN rustup target add wasm32-unknown-unknown

WORKDIR /workspace
COPY . .

RUN cargo build --target=wasm32-unknown-unknown --release

FROM envoyproxy/envoy:v1.19.1

COPY --from=builder /workspace/target/wasm32-unknown-unknown/release/envoy_example.wasm /envoy_example.wasm
