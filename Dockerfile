FROM rust:alpine as builder

WORKDIR /usr/src/gnam-api/
COPY . .
RUN cargo install --path .

FROM alpine:latest

COPY --from=builder /usr/local/cargo/bin/gnam-api /usr/local/bin/gnam-api

ENTRYPOINT ["gnam-api"]