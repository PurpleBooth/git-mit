# syntax=docker/dockerfile:1.17@sha256:e63addfe27b10e394a5f9f1e866961adc70d08573f1cb80f8d1a0999347b3553
ARG RUST_VERSION=1.87.0@sha256:251cec8da4689d180f124ef00024c2f83f79d9bf984e43c180a598119e326b84

FROM rust:${RUST_VERSION} AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:${RUST_VERSION} AS cacher
WORKDIR /app
RUN cargo install cargo-chef
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

FROM debian:bookworm-slim
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

