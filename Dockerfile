# build
FROM rust:alpine as builder

WORKDIR /app

RUN apk update
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static

COPY . .

RUN cargo build --release
# end build

# runner
FROM alpine:3.18

WORKDIR /app

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/sentry-tunneler /app/sentry-tunneler

CMD ["/app/sentry-tunneler"]
# end runner