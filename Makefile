# AeroFlow Build System

.PHONY: all compiler cli shim runtime test clean

all: compiler cli shim runtime

compiler:
	cargo build -p compiler

cli:
	cargo build -p cli

shim:
	cargo build -p shim

runtime:
	cargo build -p runtime

test:
	cargo test

clean:
	cargo clean
	rm -f out.*

# Sample Build (Flutter)
build-sample-flutter:
	cargo run -p cli -- build --target flutter samples/counter.aefl

# Sample Build (Android)
build-sample-android:
	cargo run -p cli -- build --target android samples/counter.aefl

# Sample Build (iOS)
build-sample-ios:
	cargo run -p cli -- build --target ios samples/counter.aefl

# Container Isolation Demo
demo-recursive:
	cargo run -p shim -- recursive


sample-aero:
	cargo run -p aerolang -- aerolang/examples/hello.aero
