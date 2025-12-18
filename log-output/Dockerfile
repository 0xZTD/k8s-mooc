FROM rust:1 AS builder
WORKDIR /app

COPY Cargo.toml .
COPY src/main.rs src/main.rs

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/log-output /app/log-output
ENTRYPOINT ["/app/log-output"]
