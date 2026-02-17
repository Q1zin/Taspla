FROM rust:1.93-alpine AS builder
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static curl
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release
COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/release/auth-service /usr/local/bin/
CMD ["auth-service"]