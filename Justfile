#########
# Options

opt_fmt_check := "false"
opt_lint_err := "false"
opt_doc_open := "false"

#################
# Format and lint

fmt:
	cargo fmt --all {{ if opt_fmt_check == "true" { "-- --check" } else { "" } }}

fmt-check:
	@just opt_fmt_check=true fmt

lint:
	touch src/lib.rs && cargo clippy --all --all-features {{ if opt_lint_err == "true" { "-- -D warnings" } else { "" } }}

lint-err:
	@just opt_lint_err=true lint

#######
# Tests

test:
	cargo test --all

test-day day:
	cargo test days::day{{ day }}

###############
# Documentation

doc:
	cargo doc --no-deps {{ if opt_doc_open == "true" { "--open" } else { "" } }} 

doc-open:
	@just opt_doc_open=true doc

###############
# Build and run

# Build app
build:
	cargo build

# Run all days
run-all:
	cargo run -- run-all

# Run day
run day:
	cargo run -- run {{ day }}