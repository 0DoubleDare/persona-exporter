FROM rust:1.98.1-slim-bookworm AS builder
WORKDIR /app

COPY Cargo.toml ./

RUN mkdir src/
RUN echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/persona-exporter ./persona-exporter

RUN apt-get update && apt-get install -y ca-certificates

ENTRYPOINT ["./persona-exporter"]



