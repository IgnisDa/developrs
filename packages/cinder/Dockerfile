FROM rust:1.58-buster as builder
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && \
    apt-get -y install ca-certificates cmake musl-tools libssl-dev pkg-config libpq-dev && \
    rm -rf /var/lib/apt/lists/*
COPY . .
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release --package cinder
RUN strip /target/x86_64-unknown-linux-musl/release/cinder

FROM alpine:3.8
RUN apk --no-cache add ca-certificates
ENV SERVER_HOST="0.0.0.0:80" \
    MODE="production"
WORKDIR /app
COPY --from=builder /target/x86_64-unknown-linux-musl/release/cinder ./
COPY packages/cinder/ci/CHECKS ./
EXPOSE 80

CMD ["/app/cinder"]
