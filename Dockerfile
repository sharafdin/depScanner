# Step 1: Use an official Rust image as the base for building
FROM rust:1.72 as builder

# Step 2: Set the working directory
WORKDIR /app

# Step 3: Copy the project files into the container
COPY . .

# Step 4: Build the application in release mode
RUN cargo build --release

# Step 5: Use a lightweight base image for the final runtime
FROM debian:bullseye-slim

# Step 6: Set the working directory for the runtime image
WORKDIR /app

# Step 7: Copy the binary from the builder stage
COPY --from=builder /app/target/release/depScanner /app/depScanner

# Step 8: Install required runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Step 9: Specify the command to run the binary
CMD ["./depScanner"]
