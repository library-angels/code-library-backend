FROM rust:alpine as builder

RUN apk add gcc musl-dev openssl-dev postgresql-dev
ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /usr/src
COPY . .

RUN cargo install --path api/


FROM alpine:latest
RUN apk --no-cache add libgcc

COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api

EXPOSE 8080/tcp
ENTRYPOINT [ "api" ]
