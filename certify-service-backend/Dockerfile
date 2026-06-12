# Start with image that has the Rust toolchain installed
FROM rust:1.96-alpine AS chef
USER root
# Add cargo-chef to cache dependencies
RUN apk add --no-cache musl-dev && cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
# Capture info needed to build dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin certify-service-backend

# We do not need the Rust toolchain to run the binary!
# Start with a minimal image and copy over the binary and assets folder. non-root
FROM debian:bookworm-slim AS runtime
WORKDIR /app
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/usr/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
COPY --from=builder /app/target/release/certify-service-backend /usr/local/bin
USER appuser
ENTRYPOINT ["/usr/local/bin/certify-service-backend"]
