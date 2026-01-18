# Stage 1: Build
FROM rust:1.91-bookworm AS builder

WORKDIR /usr/src/app

# Install build dependencies for OpenSSL
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifest files and build dependencies first (for layer caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy actual source code and rebuild
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install CA certificates, OpenSSL runtime libraries, and other dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/rustmail /app/rustmail

# Expose the port the app runs on
EXPOSE 3333

# Run the application
CMD ["/app/rustmail"]