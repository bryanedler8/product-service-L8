FROM rust:1.80.0 AS builder

# Create a new directory for our application
WORKDIR /product-service

# Copy the source files to the container
COPY . .

# Build the application
RUN cargo build --release

# Create a new stage and copy the binary from the builder stage
FROM debian:bookworm-slim AS runner
WORKDIR /app

# Set the build argument for the app version number
ARG APP_VERSION=0.1.0

# Install necessary dependencies for running the binary
# - ca-certificates: for HTTPS requests to AI service
# - libssl-dev: for OpenSSL support
# - pkg-config: for finding libraries (though not strictly needed at runtime)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /product-service/target/release/product-service /app/product-service

# Copy config files if needed (uncomment if you have a config.yml)
# COPY --from=builder /product-service/config.yml /app/config.yml

# Copy WASM rule engine if needed (uncomment if you have it)
# COPY --from=builder /product-service/tests/rule_engine.wasm /app/tests/rule_engine.wasm

# Set the environment variable for the app version number
ENV APP_VERSION=$APP_VERSION

# Expose the port the service runs on
EXPOSE 3002

# Set default environment variables (can be overridden at runtime)
ENV APP_HOST=0.0.0.0
ENV APP_PORT=3002
ENV LOG_LEVEL=info
ENV WASM_RULES_ENGINE_ENABLED=false
ENV AI_SERVICE_URL=http://ai-service:3001

# Run the application
CMD ["./product-service"]