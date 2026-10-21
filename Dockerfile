FROM --platform=$BUILDPLATFORM rust:latest AS builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM
WORKDIR /usr/src/chess
COPY . .
RUN ./docker-build.sh $TARGETPLATFORM

FROM debian:latest
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /usr/local/cargo/bin/lichess /usr/local/bin/lichess
CMD ["lichess"]
