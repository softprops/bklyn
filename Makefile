build:
	@cargo build

example:
	@cargo run --example metrics

test:
	@cargo test

.PHONY: build example test
