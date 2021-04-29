FROM rust:slim-buster as builder

RUN apt update && apt install -y wget gcc pkg-config libssl-dev libpq-dev

WORKDIR /usr/src
COPY . .

RUN cargo install --path book/


FROM debian:buster-slim
RUN apt update && apt install -y libpq5

COPY --from=builder /usr/local/cargo/bin/book /usr/local/bin/book

EXPOSE 8080/tcp
ENTRYPOINT ["book"]
