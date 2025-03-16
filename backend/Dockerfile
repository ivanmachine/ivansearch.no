# ---- Build Stage (Cross-platform) ----
FROM --platform=linux/amd64 rust:1.74 AS builder

# Set working directory
WORKDIR /usr/src/backend

# Install required packages for cross-compilation
RUN apt-get update && apt-get install -y build-essential pkg-config libssl-dev

# Copy dependencies first (for caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Now copy source code
COPY ./src ./src
COPY .env ./.env

# Build the actual project
RUN cargo build --release

# ---- Runtime Stage (Minimal image) ----
FROM --platform=linux/amd64 debian:bullseye-slim

# Set working directory for the app
WORKDIR /app

# Copy binary and .env file
COPY --from=builder /usr/src/backend/target/release/ivansearch-backend /app/
COPY --from=builder /usr/src/backend/.env /app/.env

# Install necessary runtime dependencies (ssl, etc.)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Expose the API port
EXPOSE 8000

# Start your Rust backend
CMD ["./ivansearch-backend"]
