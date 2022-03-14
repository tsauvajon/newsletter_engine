check:
	cargo fmt --all -- --check
	cargo test
	cargo clippy -- -D warnings
