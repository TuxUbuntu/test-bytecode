
default:
	@just --list

# tests
fullcheck: lint clippy check test-all
check:
	cargo check --all
lint:
	cargo fmt --all -- --check
clippy:
	cargo clippy
test-all:
	cargo test --all -- --nocapture
test-single CRATE:
	cargo test -p {{CRATE}} -- --nocapture
release_tests:
	cargo test --all --release -- --nocapture
format:
	cargo fmt --all
_cargo-check-deps:
	cargo +nightly udeps

# development
dev-all:
	watchexec -r -c -e rs,toml,pest -i target "clear && just test-all"
dev-single CRATE:
	watchexec -r -c -e rs,toml,pest -i target just test-single {{CRATE}}

