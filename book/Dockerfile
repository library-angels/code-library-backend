FROM rust:alpine as builder

RUN apk add gcc musl-dev postgresql-dev
ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /usr/src
COPY . .

RUN cargo install --path book/


FROM alpine:latest
RUN apk --no-cache add libgcc libpq

COPY --from=builder /usr/local/cargo/bin/book /usr/local/bin/book

EXPOSE 8080/tcp
ENTRYPOINT [ "book" ]
