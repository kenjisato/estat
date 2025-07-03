.PHONY: test test-rust test-python clean build

test: test-rust test-python

test-rust:
	@echo "Running Rust tests..."
	cd estat && cargo test

test-python:
	@echo "Building Python bindings..."
	cd estat-py && uv run maturin develop
	@echo "Running Python tests..."
	cd estat-py && uv run pytest tests/

clean:
	cd estat && cargo clean
	cd estat-py && cargo clean
	cd estat-py && rm -rf target/

build:
	cd estat && cargo build
	cd estat-py && uv run maturin develop

build-release:
	cd estat && cargo build --release
	cd estat-py && uv run maturin develop --release

help:
	@echo "Available targets:"
	@echo "  test         - Run all tests (Rust + Python)"
	@echo "  test-rust    - Run Rust tests only"
	@echo "  test-python  - Run Python tests only"
	@echo "  build        - Build both projects"
	@echo "  build-release- Build both projects in release mode"
	@echo "  clean        - Clean build artifacts"
	@echo "  help         - Show this help"
