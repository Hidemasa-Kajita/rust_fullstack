# Use the Rust image as the build stage
FROM rust:1.77.0-alpine as builder

RUN apk add --no-cache musl-dev

# RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Create a new application directory
WORKDIR /usr/src/app

# Copy the application files to the container
COPY . .

# Build the application
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release --jobs 4

# Use a minimal Alpine-based image for the final container
FROM alpine:latest as runtime

# Install OpenSSL and libpq to enable TLS and PostgreSQL support
RUN apk add --no-cache openssl-dev postgresql-dev

# Copy the compiled binary from the build stage
COPY --from=builder /usr/src/app/target/release/app /usr/local/bin/app

# Expose port 8000
EXPOSE 8000

# USER appuser

# Run the application
CMD ["/usr/local/bin/api"]
