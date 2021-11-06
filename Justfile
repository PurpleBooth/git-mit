
# This help screen
show-help:
	just --list

# Test it was built ok
test:
	unset GIT_MIT_AUTHORS_EXEC && RUST_BACKTRACE=1 cargo test

# Test the markdown in the docs directory
specdown: build
	./bin/specdown ./docs/**.md ./docs/**/*.md ./README.md

# Benchmark
bench:
	cargo bench

# Run a smoke test and see if the app runs
smoke-test: build
	cargo run --bin git-mit -- -h
	cargo run --bin git-mit-config -- -h
	cargo run --bin git-mit-relates-to -- -h
	cargo run --bin git-mit-install -- -h
	cargo run --bin mit-commit-msg -- -h
	cargo run --bin mit-pre-commit -- -h
	cargo run --bin mit-prepare-commit-msg -- -h

# Build release version
build:
	cargo build --release

# Lint it
lint:
	cargo fmt --all -- --check
	cargo +nightly clippy --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo
	cargo check
	cargo audit
	npx prettier --check **.yml **.yaml

# Publish to crates
publish-to-crates:
	( cd mit-hook-test-helper && cargo publish )
	( cd mit-commit-message-lints && cargo publish )
	( cd mit-commit-msg && cargo publish )
	( cd mit-pre-commit && cargo publish )
	( cd mit-prepare-commit-msg && cargo publish )
	( cd git-mit && cargo publish )
	( cd git-mit-config && cargo publish )
	( cd git-mit-install && cargo publish )
	( cd git-mit-relates-to && cargo publish )

# Publish to crates
release:
	versio release

# Format what can be formatted
fmt:
	cargo fix --allow-dirty --allow-staged
	cargo +nightly clippy --allow-dirty --allow-staged --fix -Z unstable-options --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo -D clippy::nursery
	cargo fmt --all
	npx prettier --write **.yml **.yaml

# Clean the build directory
clean:
	cargo clean
