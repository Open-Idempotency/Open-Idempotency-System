# Build Stage
FROM rust:1.66.0 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new Idmp-User
WORKDIR /usr/src/Idmp-User
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
COPY templates ./templates
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/Idmp-User .
USER 1000
CMD ["./Open-Idempotency", "-a", "0.0.0.0", "-p", "8080"]
