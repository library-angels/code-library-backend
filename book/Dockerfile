FROM rust:alpine as builder
LABEL stage=code-library-intermediate
WORKDIR /usr/src/code-library-book
COPY . .
RUN apk add gcc musl-dev postgresql-dev
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo install --path .

FROM alpine:latest
RUN apk add libgcc libpq
COPY --from=builder /usr/local/cargo/bin/book /usr/local/bin/book
EXPOSE 8080/tcp
CMD ["book"]
