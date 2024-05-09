FROM rust:alpine as builder

RUN apk add --no-cache musl-dev

# Add source code
WORKDIR /usr/src
COPY rest rest
COPY library library

WORKDIR /usr/src/rest
RUN cargo build --release

# Copy the binary to a new image
FROM alpine as runtime
COPY --from=builder /usr/src/rest/target/release/rest /usr/local/bin/rest
CMD ["rest"]
