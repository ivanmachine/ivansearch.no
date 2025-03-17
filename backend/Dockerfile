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

# Set the default architecture to aarch64, but allow it to be overridden
ARG TARGETARCH=aarch64

# Use the build argument to set the correct path
RUN cargo build --release
RUN chmod +x ./target/${TARGETARCH}-unknown-linux-musl/release/ivansearch-backend

EXPOSE 8000
CMD ["./target/${TARGETARCH}-unknown-linux-musl/release/ivansearch-backend"]