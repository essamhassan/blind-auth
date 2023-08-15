# Use an official Rust image as the base image
FROM rust:1.71 AS builder

RUN apt-get update \
    && apt-get install -y protobuf-compiler libprotobuf-dev

# Set the working directory inside the container
WORKDIR /usr/src/blindauth

# Copy the Rust project files (Cargo.toml and Cargo.lock) to the container
COPY Cargo.toml Cargo.lock ./

# Copy the entire source code to the container
COPY . .

# Build the Rust binary with optimizations
RUN cargo build --release

# Create a new image that only contains the binary
FROM debian:bookworm-slim

# Set the working directory inside the container
ARG APP=/usr/src/blindauth

ENV APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

# Copy the compiled binaries into the new container.
COPY --from=builder /usr/src/blindauth/target/release/prover ${APP}/prover

RUN chown -R $APP_USER:$APP_USER ${APP}
USER $APP_USER
WORKDIR ${APP}

# Run the Rust binary
CMD ["tail", "-f", "/dev/null"]
