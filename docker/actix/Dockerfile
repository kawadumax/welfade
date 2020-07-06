FROM rust:slim
WORKDIR /actix
RUN apt-get update && apt-get -y install libpq-dev
EXPOSE 3000

# ビルド時に必要
ENV USER=user

RUN cargo install diesel_cli --no-default-features --features "postgres"
RUN cargo install systemfd cargo-watch
CMD bash