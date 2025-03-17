# ---- Build Stage (Static binary using muslrust) ----
FROM clux/muslrust:stable AS builder

WORKDIR /volume

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy actual source code
COPY ./src ./src
COPY .env .env

# Build the project
RUN cargo build --release

EXPOSE 8000

# Set default target directory
ENV TARGET_DIR="x86_64-unknown-linux-musl"

# Determine architecture and set target directory
RUN arch=$(uname -m) && \
    if [ "$arch" = "x86_64" ]; then \
    TARGET_DIR="x86_64-unknown-linux-musl"; \
    elif [ "$arch" = "aarch64" ]; then \
    TARGET_DIR="aarch64-unknown-linux-musl"; \
    else \
    echo "Unsupported architecture: $arch"; \
    exit 1; \
    fi

RUN chmod +x target/${TARGET_DIR}/release/ivansearch-backend
CMD ["./target/${TARGET_DIR}/release/ivansearch-backend"]