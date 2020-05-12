FROM rust:alpine3.11
WORKDIR /usr/src/code-library-api
COPY api .
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add gcc musl-dev openssl-dev postgresql-dev
RUN cargo install --path .
CMD ["api"]