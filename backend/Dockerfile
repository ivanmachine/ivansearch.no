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

# Move the binary to the desired location
RUN chmod +x ./target/release/bin/ivansearch-backend

EXPOSE 8000
CMD ["./target/release/bin/ivansearch-backend"]