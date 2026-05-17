# syntax=docker/dockerfile:1
FROM rust:slim-bookworm AS builder

WORKDIR /app

RUN apt-get update \
 && apt-get install -y --no-install-recommends pkg-config libssl-dev \
 && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY common ./common
COPY server ./server
COPY migration ./migration
COPY toolkit ./toolkit

RUN --mount=type=cache,id=somnaire-cargo-registry-bookworm,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=somnaire-cargo-target-bookworm,target=/app/target,sharing=locked \
    cargo build --release -p server --bin server \
 && cp /app/target/release/server /usr/local/bin/server


FROM debian:bookworm-slim AS runtime

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/* \
 && useradd --system --no-create-home --shell /usr/sbin/nologin somnaire

COPY --from=builder /usr/local/bin/server /usr/local/bin/server

USER somnaire
EXPOSE 8080
CMD ["server"]
