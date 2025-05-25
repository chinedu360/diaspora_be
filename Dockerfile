# Use latest versions
FROM lukemathwalker/cargo-chef:latest-rust-1.87 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

# Plan dependencies
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build dependencies (cached layer)
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build your app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin diaspora_be

# Runtime (your version is better - bookworm vs bullseye)
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/diaspora_be diaspora_be
COPY configuration configuration
ENV APP_ENVIRONMENT=production
EXPOSE 8000
ENTRYPOINT ["./diaspora_be"]