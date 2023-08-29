FROM rust:1.72.0 as builder
LABEL authors="ttrobisch"
# Build Stage
WORKDIR /usr/src

# Copy over your Manifest files
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# This build step will cache your dependencies
RUN cargo build --release

# Final Stage
FROM debian:bookworm-slim
WORKDIR /app

# Install iperf3, libc and any required dependencies
RUN apt-get update && apt-get install -y iperf3

# Copy the binary from the builder stage
COPY --from=builder /usr/src/target/release/iperf-exporter /usr/local/bin/

# Execute the binary
CMD ["iperf-exporter"]
