FROM rust:alpine as builder

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN echo "http://dl-cdn.alpinelinux.org/alpine/edge/main" >> /etc/apk/repositories && \
    apk update && \
    apk add --no-cache musl-dev git

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine as runtime

WORKDIR /app

COPY --from=builder /app/target/release/auto-index /app/auto-index

ENTRYPOINT ["/app/auto-index", "problem"]
