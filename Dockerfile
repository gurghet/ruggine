FROM rust:slim-bookworm as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

# Build the application in release mode
RUN cargo build --release

# Create a new stage with a minimal image
FROM debian:bookworm-slim

# Install SSL certificates and SQLite for HTTPS support and database
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 libsqlite3-0 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from builder stage
COPY --from=builder /usr/src/app/target/release/url_shortener /app/

# Expose the port the app runs on
EXPOSE 3000

# Run the binary
CMD ["./url_shortener"]
