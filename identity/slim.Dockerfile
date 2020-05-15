FROM rust:alpine3.11 as builder
LABEL stage=code-library-intermediate
WORKDIR /usr/src/code-library-identity
COPY . .
RUN apk add gcc musl-dev openssl-dev
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo install --path .

FROM alpine:latest
RUN apk add gcc
COPY --from=builder /usr/local/cargo/bin/identity /usr/local/bin/identity
EXPOSE 8080/tcp
CMD ["identity"]