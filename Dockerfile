FROM rust:1.70-alpine as builder

RUN apk add musl-dev

WORKDIR /usr/src/gnam-api/
COPY . .
RUN cargo install --path .

FROM alpine:latest

RUN apk add curl

COPY --from=builder /usr/local/cargo/bin/gnam-api /usr/local/bin/gnam-api

HEALTHCHECK CMD curl --fail http://localhost:9090/health || exit 1   
ENTRYPOINT ["gnam-api"]