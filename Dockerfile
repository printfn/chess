FROM --platform=$BUILDPLATFORM rust:bookworm as builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM
WORKDIR /usr/src/chess
COPY . .
RUN ./docker-build.sh $TARGETPLATFORM

FROM debian:bookworm
COPY --from=builder /usr/local/cargo/bin/lichess /usr/local/bin/lichess
CMD ["lichess"]
