FROM rust:alpine as builder
WORKDIR /usr/src/chess
RUN apk add musl-dev
COPY . .
RUN cargo install --path lichess

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/lichess /usr/local/bin/lichess
CMD ["lichess"]
