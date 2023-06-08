
run:
	cargo run --release

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy --all-targets --all-features --workspace -- -D warnings
