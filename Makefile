# Makefile for key-listener
.PHONY: all clean build build-all build-direct build-complex build-hold-release build-simulation build-mouse help

# Default target
all: build-all

# Build all variants
build-all:
	@echo "Building all key-listener variants..."
	@./build.sh

# Build default (all features)
build:
	@echo "Building key-listener with all features..."
	cargo build --release

# Build individual feature variants
build-direct:
	@echo "Building direct-only binary..."
	cargo build --release --bin key-listener-direct --features direct --no-default-features

build-complex:
	@echo "Building complex-only binary..."
	cargo build --release --bin key-listener-complex --features complex --no-default-features

build-hold-release:
	@echo "Building hold-and-release-only binary..."
	cargo build --release --bin key-listener-hold-release --features hold_and_release --no-default-features

build-simulation:
	@echo "Building simulation-only binary..."
	cargo build --release --bin key-listener-simulation --features simulation --no-default-features

build-mouse:
	@echo "Building mouse-only binary..."
	cargo build --release --bin key-listener-mouse --features mouse --no-default-features

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Show help
help:
	@echo "Available targets:"
	@echo "  all              - Build all variants (default)"
	@echo "  build            - Build main binary with all features"
	@echo "  build-all        - Build all variants using build script"
	@echo "  build-direct     - Build direct-only binary"
	@echo "  build-complex    - Build complex-only binary"
	@echo "  build-hold-release - Build hold-and-release-only binary"
	@echo "  build-simulation - Build simulation-only binary"
	@echo "  build-mouse      - Build mouse-only binary"
	@echo "  clean            - Clean build artifacts"
	@echo "  help             - Show this help message"
