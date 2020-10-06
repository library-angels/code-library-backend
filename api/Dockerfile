FROM rust:alpine as builder
LABEL stage=code-library-intermediate

RUN apk add gcc musl-dev openssl-dev postgresql-dev

WORKDIR /usr/src
COPY api/ api/
COPY identity/ identity/

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo install --path api/


FROM alpine:latest
RUN apk --no-cache add libgcc libpq

COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api

EXPOSE 8080/tcp
ENTRYPOINT [ "api"]
