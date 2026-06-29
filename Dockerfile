# =====================
# Stage 1: Build the Rust backend
# =====================
ARG REGISTRY_MIRROR=
FROM ${REGISTRY_MIRROR}rust:1.88-slim-bookworm AS builder

WORKDIR /app

# Install required build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifest files first for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release 2>&1 || true

# Now copy the actual source code
COPY src ./src
COPY .sqlx ./.sqlx

# Build the actual application
RUN cargo build --release --bin rustyexpress

# =====================
# Stage 2: Build the frontend (static files - just copy as-is)
# =====================
ARG REGISTRY_MIRROR=
FROM ${REGISTRY_MIRROR}nginx:alpine AS frontend

# Copy frontend files into nginx's html directory
COPY frontend /usr/share/nginx/html

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

# =====================
# Stage 3: Runtime stage
# =====================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rustyexpress /app/rustyexpress

EXPOSE 8080

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
ENV DB_MAX_CONNECTIONS=10
ENV JWT_EXPIRATION_HOURS=168
ENV RUST_LOG=info

CMD ["/app/rustyexpress"]

