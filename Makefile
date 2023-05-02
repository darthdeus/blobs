.PHONY: demo test

FEATURES=jemalloc

default: demo
# default: test

demo:
	RUST_BACKTRACE=1 cargo run --bin demo --features $(FEATURES)

test:
	cargo test
