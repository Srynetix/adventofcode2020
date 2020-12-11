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

# Generate documentation
doc:
	cargo doc --no-deps

# Generate documentation and open target file in browser
doc-open:
	cargo doc --no-deps --open