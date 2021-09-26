FROM rust:1.55.0 as builder

WORKDIR /root/app

## Build deps for git-mit
RUN apt-get update && \
    apt-get install -y libxkbcommon-dev libxcb-shape0-dev libxcb-xfixes0-dev pandoc && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN make build generate-manpages

FROM debian:11.0
ENV DEBIAN_FRONTEND noninteractive

### Nice things if for actually using the tool
## Git
RUN apt-get update && \
    apt-get install -y git && \
    rm -rf /var/lib/apt/lists/*

## Bash
RUN apt-get update && \
    apt-get install -y bash && \
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
    apt-get install -y libxkbcommon0 libxcb-shape0 libxcb-xfixes0 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /root/app/target/release/mit-commit-msg /usr/local/bin/mit-commit-msg
COPY --from=builder /root/app/target/release/build/mit-commit-msg-*/out/bash_completion/mit-commit-msg.bash /usr/local/share/bash-completion/completions/mit-commit-msg
COPY --from=builder /root/app/target/release/mit-pre-commit /usr/local/bin/mit-pre-commit
COPY --from=builder /root/app/target/release/build/mit-pre-commit-*/out/bash_completion/mit-pre-commit.bash /usr/local/share/bash-completion/completions/mit-pre-commit
COPY --from=builder /root/app/target/release/mit-prepare-commit-msg /usr/local/bin/mit-prepare-commit-msg
COPY --from=builder /root/app/target/release/build/mit-prepare-commit-msg-*/out/bash_completion/mit-prepare-commit-msg.bash /usr/local/share/bash-completion/completions/mit-prepare-commit-msg
COPY --from=builder /root/app/target/release/git-mit /usr/local/bin/git-mit
COPY --from=builder /root/app/target/release/build/git-mit-*/out/bash_completion/git-mit.bash /usr/local/share/bash-completion/completions/git-mit
COPY --from=builder /root/app/target/release/git-mit-config /usr/local/bin/git-mit-config
COPY --from=builder /root/app/target/release/build/git-mit-config-*/out/bash_completion/git-mit-config.bash /usr/local/share/bash-completion/completions/git-mit-config
COPY --from=builder /root/app/target/release/git-mit-relates-to /usr/local/bin/git-mit-relates-to
COPY --from=builder /root/app/target/release/build/git-mit-relates-to-*/out/bash_completion/git-mit-relates-to.bash /usr/local/share/bash-completion/completions/git-mit-relates-to
COPY --from=builder /root/app/target/release/git-mit-install /usr/local/bin/git-mit-install
COPY --from=builder /root/app/target/release/build/git-mit-install-*/out/bash_completion/git-mit-install.bash /usr/local/share/bash-completion/completions/git-mit-install
COPY --from=builder /root/app/target/*.1 /usr/local/share/man/man1/
RUN git-mit-install --scope=global

