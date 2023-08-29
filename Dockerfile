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
FROM alpine:3.18.3
WORKDIR /app

# Install iperf3
RUN apk --no-cache add iperf3

# Copy the binary from the builder stage
COPY --from=builder /usr/src/target/release/iperf-exporter .

# Expose the port
EXPOSE 3030

# Command to run the binary
CMD ["./iperf-exporter"]
