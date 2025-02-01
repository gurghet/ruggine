# Use the official cargo-chef image with latest Rust
FROM lukemathwalker/cargo-chef:latest AS chef
# Enable parallel compilation
ENV CARGO_BUILD_JOBS=8
WORKDIR app

FROM chef AS planner
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

# Create a new stage with a minimal image
FROM debian:bookworm-slim

# Install SSL certificates and SQLite for HTTPS support and database
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 libsqlite3-0 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from builder stage
COPY --from=builder /app/target/release/url_shortener /app/

# Expose the port the app runs on
EXPOSE 3000

# Run the binary
CMD ["./url_shortener"]
