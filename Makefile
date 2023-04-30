.PHONY: demo test

default: demo
# default: test

demo:
	cargo run --bin demo

test:
	cargo test
