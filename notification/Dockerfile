FROM rust:slim-buster as builder

RUN apt update && apt install -y wget gcc pkg-config libssl-dev libpq-dev

WORKDIR /usr/src
COPY . .

RUN cargo install --path notification/


FROM debian:buster-slim
RUN apt update && apt install -y libpq5

COPY --from=builder /usr/local/cargo/bin/notification /usr/local/bin/notification

EXPOSE 8080/tcp
ENTRYPOINT ["notification"]
