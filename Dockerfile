FROM rust:latest as chef
WORKDIR /app
RUN apt-get -qy update \
    && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    && apt-get clean autoclean \
    && apt-get -y autoremove \
    && rm -Rf /var/lib/apt/* /var/cache/* /var/log/*
RUN cargo install cargo-chef

ARG MODULE

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src/${MODULE} src/${MODULE}
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --release
COPY Cargo.toml Cargo.lock ./
COPY proto proto/
COPY src/${MODULE} src/${MODULE}
RUN cargo build --bin ${MODULE} --release
RUN mv target/release/${MODULE} bin

FROM debian:bookworm-slim as runtime
WORKDIR /app
COPY --from=builder /app/bin bin
ENTRYPOINT ["/app/bin"]
