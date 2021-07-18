# 開発環境
FROM rust:1.52-buster as develop-stage
RUN apt-get update & apt-get install git
ENV RUST_BACKTRACE=1
WORKDIR /app/root
RUN cargo install cargo-watch
RUN rustup component add rust-src clippy rustfmt
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-edit
COPY . .

# ビルド環境
FROM develop-stage as build-stage
WORKDIR /app/root
COPY ./root /app/root/
RUN cargo build --release

# 本番環境
FROM rust:1.52-slim-buster
COPY --from=build-stage /app/root/target/release/root /usr/local/bin/root
EXPOSE 8000
CMD ["/usr/local/bin/root"]
