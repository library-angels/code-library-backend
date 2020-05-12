FROM rust:alpine3.11
WORKDIR /usr/src/code-library-notification
COPY notification .
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add gcc musl-dev
RUN cargo install --path .
CMD ["notification"]