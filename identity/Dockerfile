FROM rust:alpine as builder

RUN apk add gcc musl-dev openssl-dev postgresql-dev
ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /usr/src
COPY . .

RUN cargo install --path identity/


FROM alpine:latest
RUN apk --no-cache add libgcc libpq

COPY --from=builder /usr/local/cargo/bin/identity /usr/local/bin/identity

EXPOSE 8080/tcp
ENTRYPOINT [ "identity" ]
