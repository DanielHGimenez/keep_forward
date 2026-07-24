BINARY = keep_forward

.PHONY: all build test windows clean

all: build

build:
	cargo build --release

test:
	cargo test

# Cross-compile to Windows. Requires: rustup target add x86_64-pc-windows-gnu
windows:
	cargo build --release --target x86_64-pc-windows-gnu

clean:
	cargo clean
