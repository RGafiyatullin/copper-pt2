FROM rust:1.66 AS build

RUN mkdir /src
COPY apps /src/apps
COPY libs /src/libs
COPY Cargo.toml /src/Cargo.toml
COPY Cargo.lock /src/Cargo.lock

WORKDIR /src

RUN cargo build --release

FROM rust:1.66

RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        libssl1.1

ARG COMPONENT

COPY --from=build /src/target/release/$COMPONENT /app/bin/$COMPONENT

# COPY --from=build /src/target /app/.build-release-target

