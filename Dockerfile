FROM rust:1.58.0 as builder

## Update the system generally
RUN apt-get update && \
    apt-get upgrade -y && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /root/app

## Build deps for git-mit
RUN apt-get update && \
    apt-get install -y libxkbcommon-dev libxcb-shape0-dev libxcb-xfixes0-dev help2man && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN --mount=type=cache,target=/root/.cargo cargo clean
RUN --mount=type=cache,target=/root/.cargo cargo build --release
RUN help2man target/release/mit-commit-msg > target/mit-commit-msg.1
RUN help2man target/release/mit-pre-commit > target/mit-pre-commit.1
RUN help2man target/release/mit-prepare-commit-msg > target/mit-prepare-commit-msg.1
RUN help2man target/release/git-mit > target/git-mit.1
RUN help2man target/release/git-mit-config > target/git-mit-config.1
RUN help2man target/release/git-mit-relates-to > target/git-mit-relates-to.1
RUN help2man target/release/git-mit-install > target/git-mit-install.1

FROM rust:1.58.0
ENV DEBIAN_FRONTEND noninteractive

## Update the system generally
RUN apt-get update && \
    apt-get upgrade -y && \
    rm -rf /var/lib/apt/lists/*

### Nice things if for actually using the tool
## Bash
RUN apt-get update && \
    apt-get install -y bash bash-completion && \
    rm -rf /var/lib/apt/lists/*

## Git
RUN apt-get update && \
    apt-get install -y git && \
    rm -rf /var/lib/apt/lists/*

## Vim
RUN apt-get update && \
    apt-get install -y vim && \
    rm -rf /var/lib/apt/lists/*

## Man
RUN apt-get update && \
    apt-get install -y man && \
    rm -rf /var/lib/apt/lists/*

### The Tool
## Runtime deps for git-mit
RUN apt-get update && \
    apt-get install -y libxkbcommon0 libxcb-shape0 libxcb-xfixes0 libssl1.1 libgcc1 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder \
    /root/app/target/release/mit-commit-msg \
    /usr/local/bin/mit-commit-msg
COPY --from=builder \
    /root/app/target/release/mit-pre-commit \
    /usr/local/bin/mit-pre-commit
COPY --from=builder \
    /root/app/target/release/mit-prepare-commit-msg \
    /usr/local/bin/mit-prepare-commit-msg
COPY --from=builder \
    /root/app/target/release/git-mit \
    /usr/local/bin/git-mit
COPY --from=builder \
    /root/app/target/release/git-mit-config \
    /usr/local/bin/git-mit-config
COPY --from=builder \
    /root/app/target/release/git-mit-relates-to \
    /usr/local/bin/git-mit-relates-to
COPY --from=builder \
    /root/app/target/release/git-mit-install \
    /usr/local/bin/git-mit-install
COPY --from=builder \
    /root/app/target/*.1 \
    /usr/local/share/man/man1/

RUN mkdir -p $HOME/.local/share/bash-completion/completions
RUN mit-commit-msg --completion bash > $HOME/.local/share/bash-completion/completions/mit-commit-msg
RUN mit-pre-commit --completion bash > $HOME/.local/share/bash-completion/completions/mit-pre-commit
RUN mit-prepare-commit-msg --completion bash > $HOME/.local/share/bash-completion/completions/mit-prepare-commit-msg
RUN git-mit --completion bash > $HOME/.local/share/bash-completion/completions/git-mit
RUN git-mit-config --completion bash > $HOME/.local/share/bash-completion/completions/git-mit-config
RUN git-mit-relates-to --completion bash > $HOME/.local/share/bash-completion/completions/git-mit-relates-to
RUN git-mit-install --completion bash > $HOME/.local/share/bash-completion/completions/git-mit-install

RUN git-mit-install --scope=global

