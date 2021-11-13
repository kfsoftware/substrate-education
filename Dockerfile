# 1. This tells docker to use the Rust official image
FROM rust:1.56

# 2. Copy the files in your machine to the Docker image


# Build your program for release
COPY ./target/release/node-kitties ./target/release/node-kitties

# Run the binary
ENTRYPOINT ["./target/release/node-kitties"]