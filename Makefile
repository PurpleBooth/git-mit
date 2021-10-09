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
	unset GIT_MIT_AUTHORS_EXEC && RUST_BACKTRACE=1 cargo test

.PHONY: specdown
## Test the markdown in the docs directory
specdown: build
	./bin/specdown ./docs/**.md ./docs/**/*.md ./README.md

.PHONY: bench
## Benchmark
bench:
	cargo bench

.PHONY: smoke-test
## Run a smoke test and see if the app runs
smoke-test: build
	cargo run --bin git-mit -- -h
	cargo run --bin git-mit-config -- -h
	cargo run --bin git-mit-relates-to -- -h
	cargo run --bin git-mit-install -- -h
	cargo run --bin mit-commit-msg -- -h
	cargo run --bin mit-pre-commit -- -h
	cargo run --bin mit-prepare-commit-msg -- -h

.PHONY: build
## Build release version
build:
	cargo build --release

.PHONY: lint
## Lint it
lint:
	cargo fmt --all -- --check
	cargo +nightly clippy --all-features -- -D warnings -Dclippy::all -D clippy::pedantic
	cargo check
	cargo audit --ignore  RUSTSEC-2021-0119
	npx prettier --check **.yml **.yaml

.PHONY: publish-to-crates
## Publish to crates
publish-to-crates:
	( cd mit-build-tools && cargo publish )
	( cd mit-hook-test-helper && cargo publish )
	( cd mit-commit-message-lints && cargo publish )
	( cd mit-commit-msg && cargo publish )
	( cd mit-pre-commit && cargo publish )
	( cd mit-prepare-commit-msg && cargo publish )
	( cd git-mit && cargo publish )
	( cd git-mit-config && cargo publish )
	( cd git-mit-install && cargo publish )
	( cd git-mit-relates-to && cargo publish )

.PHONY: release
## Publish to crates
release:
	versio release

.PHONY: fmt
## Format what can be formatted
fmt:
	cargo fix --allow-dirty --allow-staged
	cargo +nightly clippy --allow-dirty --allow-staged --fix -Z unstable-options --all-features -- -D warnings -Dclippy::all -D clippy::pedantic
	cargo fmt --all
	npx prettier --write **.yml **.yaml

.PHONY: clean
## Clean the build directory
clean:
	cargo clean

.PHONY: generate-manpages
## Build man pages from templates
generate-manpages: build
	./bin/generate-manpages
