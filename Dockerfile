# syntax=docker/dockerfile:1.17@sha256:38387523653efa0039f8e1c89bb74a30504e76ee9f565e25c9a09841f9427b05
ARG RUST_VERSION=1.96.0@sha256:6df234c1eb92b0545468fab8c18fc5f9adfb994e7d4f67d81d45fe2fcabf5657

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

FROM debian:bookworm-slim@sha256:96e378d7e6531ac9a15ad505478fcc2e69f371b10f5cdf87857c4b8188404716
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

