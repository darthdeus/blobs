.PHONY: demo test

FEATURES=jemalloc

default: demo
# default: test

demo:
	cargo run --bin demo --features $(FEATURES)

test:
	cargo test
