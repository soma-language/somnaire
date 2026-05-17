# syntax=docker/dockerfile:1.7
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

RUN --mount=type=cache,id=somnaire-cargo-registry,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=somnaire-cargo-target,target=/app/target,sharing=locked \
    cargo build --release -p migration --bin migration \
 && cp /app/target/release/migration /usr/local/bin/migration


FROM debian:bookworm-slim AS runtime

RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/bin/migration /usr/local/bin/migration

ENTRYPOINT ["migration"]
CMD ["up"]
