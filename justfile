default:
	@just --list

alias b := build
alias d := docs
alias di := docs-internal
alias t := test

build:
	cargo build --workspace

check:
	cargo check --workspace

clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
	cargo +nightly fmt --all

fmt-check:
	cargo +nightly fmt --all -- --check

sort:
	cargo sort --workspace

docs:
	cargo doc --workspace --open

docs-internal:
	cargo doc --workspace --open --document-private-items

test:
	cargo test --workspace
