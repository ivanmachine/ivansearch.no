# Use the latest official Rust image
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/backend

# Install the target architecture
RUN rustup target add x86_64-unknown-linux-gnu

# Copy the Cargo files and install dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Copy the actual source code and build
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Use a minimal image for runtime with a newer glibc
FROM debian:bookworm-slim
WORKDIR /app

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /usr/src/backend/target/x86_64-unknown-linux-gnu/release/backend /app/

# Expose port
EXPOSE 3000

# Start the server with better error visibility
CMD ["./backend"]
