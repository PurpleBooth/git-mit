.DEFAULT_GOAL := show-help
THIS_FILE := $(lastword $(MAKEFILE_LIST))
ROOT_DIR:=$(shell dirname $(realpath $(THIS_FILE)))

.PHONY: show-help
# See <https://gist.github.com/klmr/575726c7e05d8780505a> for explanation.
## This help screen
show-help:
	@echo "$$(tput bold)Available rules:$$(tput sgr0)";echo;sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## //;td" -e"s/:.*//;G;s/\\n## /---/;s/\\n/ /g;p;}" ${MAKEFILE_LIST}|LC_ALL='C' sort -f|awk -F --- -v n=$$(tput cols) -v i=29 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"%s%*s%s ",a,-i,$$1,z;m=split($$2,w," ");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;printf"\n%*s ",-i," ";}printf"%s ",w[j];}printf"\n";}'

.PHONY: test
## Test it was built ok
test:
	RUST_BACKTRACE=1 cargo test

.PHONY: smoke-test
## Run a smoke test and see if the app runs
smoke-test:
	cargo run --bin pb-prepare-commit-msg -- -h
	cargo run --bin pb-pre-commit -- -h
	cargo run --bin pb-commit-msg -- -h

.PHONY: build
## Build release version
build:
	cargo build --release

.PHONY: lint
## Lint it
lint:
	cargo fmt --all -- --check
	cargo clippy --all-features -- -D warnings -Dclippy::style -D clippy::pedantic -D clippy::cargo
	cargo check

.PHONY: fmt
## Format what can be formatted
fmt:
	cargo fmt --all
	cargo fix --allow-dirty
	cargo clippy --allow-dirty --fix -Z unstable-options --all-features -- -D warnings -Dclippy::style -D clippy::pedantic -D clippy::cargo

.PHONY: clean
## Clean the build directory
clean:
	cargo clean
