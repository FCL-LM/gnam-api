FROM rust:1.70-bookworm as builder

WORKDIR /usr/src/gnam-api/
COPY . .
RUN cargo install --path .

FROM debian:bookworm

RUN apt-get update && \
    apt-get -y install curl

COPY --from=builder /usr/local/cargo/bin/gnam-api /usr/local/bin/gnam-api

HEALTHCHECK CMD curl --fail http://localhost:9090/health || exit 1   
ENTRYPOINT ["gnam-api"]