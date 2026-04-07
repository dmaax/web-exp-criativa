# ============================================
# Stage 1 — Builder
# ============================================
FROM rust:1.85-bookworm AS builder

RUN apt-get update && apt-get install -y \
    libpq-dev \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install diesel_cli (only postgres feature) for running migrations
RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

# Cache dependencies: copy manifests first, create dummy main, build, then swap real source
COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo 'fn main() { println!("dummy"); }' > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Now copy the real source and rebuild (only the app crate is recompiled)
COPY src ./src

RUN touch src/main.rs && cargo build --release

# ============================================
# Stage 2 — Runtime (slim)
# ============================================
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libpq5 \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Binary
COPY --from=builder /app/target/release/web-exp-criativa ./web-exp-criativa

# Diesel CLI (for migrations at startup)
COPY --from=builder /usr/local/cargo/bin/diesel ./diesel

# Migrations & diesel config
COPY migrations ./migrations
COPY diesel.toml ./diesel.toml

# Static assets served by Rocket
COPY static ./static

# Entrypoint that runs migrations then starts the app
COPY entrypoint.sh ./entrypoint.sh
RUN chmod +x entrypoint.sh

# Rocket defaults — listen on all interfaces so Docker can reach the container
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

EXPOSE 8000

ENTRYPOINT ["./entrypoint.sh"]
