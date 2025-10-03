FROM rust:1.84 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY services/users/Cargo.toml ./services/users/

RUN mkdir -p services/users/src && echo "fn main() {}" > services/users/src/main.rs
RUN cargo build --release -p users && rm -rf services/users/src

COPY services/users/src ./services/users/src

RUN cargo build --release -p users

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/users /app/users

EXPOSE 8080

CMD ["/app/users"]
