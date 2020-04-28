FROM rust:alpine3.11
WORKDIR /usr/src/code-library-api
COPY . .
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add gcc musl-dev openssl-dev postgresql-dev
RUN cargo install --path .
CMD ["api"]