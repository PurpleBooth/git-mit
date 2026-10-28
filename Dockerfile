# syntax=docker/dockerfile:1.27@sha256:4edf897a3ffa55b89f906fc8cc78afdb3f1834cc9c7083565e611a8a7d5fe99e
ARG RUST_VERSION=1.99.1@sha256:09027f873d45b8ecd1dc86f1fa13b7a62b875c961f017a0e331267b350430137

FROM rust:${RUST_VERSION} AS planner
WORKDIR /app
RUN cargo install cargo-chef --locked
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:${RUST_VERSION} AS cacher
WORKDIR /app
RUN cargo install cargo-chef --locked
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:${RUST_VERSION} AS builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN apt-get update && \
    apt-get install -y help2man && \
    rm -rf /var/lib/apt/lists/*
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release && \
    for bin in mit-commit-msg mit-pre-commit mit-prepare-commit-msg git-mit git-mit-config git-mit-relates-to git-mit-install; do \
        help2man target/release/$bin > target/$bin.1; \
    done

FROM debian:bookworm-slim@sha256:7ff6ba31527b9e336162fa061746c366c4b2032f4d2cff2078bea43f78efe3f8
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    git \
    bash \
    bash-completion \
    man \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/* /usr/local/bin/
COPY --from=builder /app/target/*.1 /usr/local/share/man/man1/

RUN mkdir -p /usr/share/bash-completion/completions && \
    for bin in mit-commit-msg mit-pre-commit mit-prepare-commit-msg git-mit git-mit-config git-mit-relates-to git-mit-install; do \
        $bin --completion bash > /usr/share/bash-completion/completions/$bin; \
    done && \
    git-mit-install --scope=global

