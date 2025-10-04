FROM rust:1.84 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY shared ./shared
COPY services/users ./services/users
COPY services/events ./services/events

RUN cargo build --release -p events

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/events /app/events

EXPOSE 8081

CMD ["/app/events"]
