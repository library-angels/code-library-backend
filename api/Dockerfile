FROM rust
RUN USER=root cargo new --bin api

WORKDIR /api
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build
RUN rm src/*.rs

COPY ./src ./src
COPY ./diesel.toml ./diesel.toml

RUN rm ./target/debug/deps/api*
RUN cargo build --release

CMD ["./target/debug/api"]
EXPOSE 8000

