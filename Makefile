CARGO ?= cargo
FEATURES ?=

.PHONY: fmt lint check test doc cli

fmt:
	$(CARGO) fmt --all

lint:
	$(CARGO) clippy --workspace --all-targets --features "$(FEATURES)" -D warnings

check:
	$(CARGO) check --workspace --features "$(FEATURES)"

# Unit + integration tests (golden tests currently ignored).
test:
	$(CARGO) test --workspace --features "$(FEATURES)"

doc:
	$(CARGO) doc --workspace --no-deps --features "$(FEATURES)"

# Build the optional CLI tool.
cli:
	$(CARGO) build -p eth-config-cli --features "$(FEATURES)"
