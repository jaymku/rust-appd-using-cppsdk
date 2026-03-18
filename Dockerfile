# --- Build Stage ---
# linux/amd64 (x86_64); Debian Bookworm has glibc 2.31 (>= 2.5 required by AppD C++ SDK)
# Use Rust 1.88+ (home crate 0.5.12 and bindgen deps require it)
FROM rust:1.88-slim-bookworm as builder

# Install clang for bindgen (to parse C++ headers)
RUN apt-get update && apt-get install -y \
    clang \
    libclang-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Copy the SDK first (matches your tree structure)
COPY appdynamics-cpp-sdk/ ./appdynamics-cpp-sdk/

# 2. Copy the rest of the project files
COPY Cargo.toml build.rs wrapper.h ./
COPY src/ ./src/

# 3. Build the application
# This triggers build.rs, which runs bindgen in C++ mode
RUN cargo build --release

# --- Runtime Stage ---
# Same glibc family as builder; AppD C++ SDK supports Linux x64/x86 with libc >= 2.5
FROM --platform=linux/amd64 debian:bookworm-slim

WORKDIR /usr/local/bin

# Install CA certificates for SSL connection to AppD Controller
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder
COPY --from=builder /app/target/release/appd-rust-app .

# Copy the SDK shared library to system lib path
COPY --from=builder /app/appdynamics-cpp-sdk/lib/libappdynamics.so /usr/local/lib/

# Update the dynamic linker cache
RUN ldconfig
ENV LD_LIBRARY_PATH=/usr/local/lib

CMD ["./appd-rust-app"]
