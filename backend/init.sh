arch=$(uname -m) && \
    if [ "$arch" = "x86_64" ]; then \
    TARGET_DIR="x86_64-unknown-linux-musl"; \
    elif [ "$arch" = "aarch64" ]; then \
    TARGET_DIR="aarch64-unknown-linux-musl"; \
    else \
    echo "Unsupported architecture: $arch"; \
    exit 1; \
    fi && \

chmod +x target/${TARGET_DIR}/release/ivansearch-backend
sh -c "./target/${TARGET_DIR}/release/ivansearch-backend"