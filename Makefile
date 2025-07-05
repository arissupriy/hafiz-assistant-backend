# File: Makefile

ANDROID_TARGET = aarch64-linux-android
ANDROID_OUT = ./jniLibs

all: help

build-android:
	@echo "📦 Building Android .so for all common ABIs..."
	cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 -o $(ANDROID_OUT) build --release

build-wasm:
	@echo "🌐 Building WebAssembly..."
	cargo build --release --target wasm32-unknown-unknown --features wasm
	wasm-bindgen target/wasm32-unknown-unknown/release/my_rust_lib.wasm \
		--out-dir pkg --target web

build-cli:
	@echo "🧰 Building CLI..."
	cargo build --release --bin hafiz_assistant_engine_cli

clean:
	cargo clean
	rm -rf pkg $(ANDROID_OUT)

help:
	@echo "📌 Available commands:"
	@echo "  make build-android   - Build for Android (.so)"
	@echo "  make build-wasm      - Build for WebAssembly"
	@echo "  make clean           - Clean build outputs"
