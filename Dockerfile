FROM --platform=$BUILDPLATFORM rust:1-slim-bullseye AS vendor

RUN cargo init --bin /code
WORKDIR /code
COPY Cargo.lock Cargo.toml /code/
RUN mkdir -p /code/.cargo && cargo vendor > /code/.cargo/config

FROM rust:1-slim-bullseye

ENV USER=root

COPY --from=vendor /code /code
COPY src/payload.json src/main.rs /code/src/

WORKDIR /code
RUN cargo run --offline
