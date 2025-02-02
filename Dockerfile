FROM lukemathwalker/cargo-chef:latest AS chef
ENV CARGO_BUILD_JOBS=8
ENV CARGO_INCREMENTAL=1
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
        libsqlite3-0 && \
    rm -rf /var/lib/apt/lists/* && \
    apt-get clean

WORKDIR /app

RUN useradd -r -u 1001 appuser
USER appuser

COPY --from=builder --chmod=755 /app/target/release/url_shortener /app/

EXPOSE 3000

ENTRYPOINT ["./url_shortener"]