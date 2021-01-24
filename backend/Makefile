BACKEND_SRC_FILES := $(shell find . -type f -name "*.rs")
DEBUG_BACKEND_TARGET := ./target/debug/korean_gongbu

clippy:
	cargo clippy

fmt:
	cargo +nightly fmt

build: $(DEBUG_BACKEND_TARGET)

$(DEBUG_BACKEND_TARGET): Cargo.toml $(BACKEND_SRC_FILES)
	cargo +nightly build
