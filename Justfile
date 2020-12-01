# Variables
day := "00"

# Format code
fmt:
	cargo fmt --all

# Check if format is correct
fmt-check:
	cargo fmt --all -- --check

# Build application
build:
	cargo build

# Lint code
lint:
	touch src/lib.rs && cargo clippy --all --all-features -- -D warnings

# Test application
test:
	cargo test --all

doc:
	cargo doc --no-deps --open

run-day:
	cargo run -- run day{{ day }}