FROM rust:alpine3.11 as builder
LABEL stage=code-library-intermediate
WORKDIR /usr/src/code-library-borrow
COPY borrow .
RUN apk add gcc musl-dev
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo install --path .

FROM alpine:latest
RUN apk add gcc
COPY --from=builder /usr/local/cargo/bin/borrow /usr/local/bin/borrow
EXPOSE 8080/tcp
CMD ["borrow"]