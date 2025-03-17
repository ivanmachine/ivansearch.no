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
COPY init.sh init.sh

# Build the project
RUN cargo build --release

EXPOSE 8000
# Determine architecture and set target directory
RUN chmod +x init.sh
CMD ["/bin/sh", "./init.sh"]