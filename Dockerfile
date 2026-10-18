# syntax=docker/dockerfile:1

# ---------- build ----------
# Edition 2024 needs Rust >= 1.85; the `1-slim-trixie` tag tracks the latest 1.x
# on Debian trixie, which also matches the runtime image's glibc.
FROM rust:1-slim-trixie AS builder

# openssl-sys (via sea-orm's native-tls) needs pkg-config + libssl-dev.
# aws-lc-sys (via aws-sdk-s3/rustls) needs cmake; clang/libclang are needed if it
# falls back to bindgen on this target.
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        pkg-config \
        libssl-dev \
        cmake \
        clang \
        libclang-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Optional comma/space separated feature list, e.g. --build-arg CARGO_FEATURES=response-compression
ARG CARGO_FEATURES=""

# Build dependencies first with dummy sources so they stay cached when only
# application code changes.
COPY Cargo.toml Cargo.lock ./
COPY api-types/Cargo.toml api-types/Cargo.toml
COPY entity/Cargo.toml entity/Cargo.toml
COPY migration/Cargo.toml migration/Cargo.toml
RUN mkdir -p src api-types/src entity/src migration/src \
    && echo 'fn main() {}' > src/main.rs \
    && : > api-types/src/lib.rs \
    && : > entity/src/lib.rs \
    && : > migration/src/lib.rs \
    && cargo build --release --locked ${CARGO_FEATURES:+--features "$CARGO_FEATURES"}

# Real sources; this only rebuilds the workspace crates.
COPY . .
# COPY preserves the sources' (older) mtimes, so cargo would still consider the
# dummy crates fresh and ship the do-nothing `fn main() {}` binary, which exits
# 0 immediately: a silent restart loop with no logs. Dropping only the workspace
# crates' artifacts forces them to rebuild while keeping the dependency cache.
RUN cargo clean -p sync_server -p api-types -p entity -p migration \
    && cargo build --release --locked ${CARGO_FEATURES:+--features "$CARGO_FEATURES"}

# ---------- runtime ----------
FROM debian:trixie-slim AS runtime

# ca-certificates: HTTPS to S3 (and anywhere else).
# libssl3t64: runtime link dependency of the natively-linked openssl
# (the t64 suffix is trixie's time_t-transition package name).
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3t64 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system --gid 10001 app \
    && useradd --system --uid 10001 --gid app --home-dir /app --shell /usr/sbin/nologin app

WORKDIR /app

COPY --from=builder /build/target/release/sync_server /usr/local/bin/sync_server

# The server refuses to start without a `.env` file (dotenvy::dotenv()? in main.rs)
# and without the local storage directory. Both are created here; the empty .env
# means configuration can be supplied purely through `docker run -e ...`.
# config.json is generated on first start (default: local storage at ./storage).
RUN mkdir -p /app/storage \
    && : > /app/.env \
    && chown -R app:app /app

USER app

ENV HOST=0.0.0.0 \
    PORT=3000

EXPOSE 3000

# Persist local-storage blobs. Ignored when config.json selects the S3 backend.
VOLUME ["/app/storage"]

# exec form: sync_server is PID 1 and receives SIGTERM directly.
CMD ["sync_server"]
