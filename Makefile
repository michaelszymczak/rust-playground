build:
	cargo check && cargo test --all && cargo build --release --all-features
