# Build Guide - Hafiz Assistant Backend

## Overview
Proyek ini mendukung berbagai target build dengan konfigurasi yang terpisah untuk memastikan output yang tepat untuk setiap platform.

## Build Targets

### 1. Android Library Build
```bash
cargo make build-android
```
- **Output**: File `.so` library untuk Android
- **Lokasi**: `./jniLibs/`
- **Arsitektur**: armeabi-v7a, arm64-v8a, x86, x86_64
- **Fitur**: Hanya library (tanpa CLI binary)
- **Konfigurasi**: `--lib --no-default-features`

**Yang dihasilkan:**
```
jniLibs/
├── arm64-v8a/
│   └── libhafiz_assistant_engine.so
├── armeabi-v7a/
│   └── libhafiz_assistant_engine.so
├── x86/
│   └── libhafiz_assistant_engine.so
└── x86_64/
    └── libhafiz_assistant_engine.so
```

### 2. CLI Binary Build (Desktop)
```bash
cargo make build-cli
```
- **Output**: Executable binary untuk desktop
- **Lokasi**: `./target/release/`
- **Platform**: Windows, Linux, macOS
- **Fitur**: CLI interface dengan feature `cli`

### 3. WebAssembly Build
```bash
cargo make build-wasm
```
- **Output**: WebAssembly module
- **Lokasi**: `./pkg/`
- **Fitur**: Web integration dengan feature `wasm`

### 4. Build All
```bash
cargo make build-all
```
- Membangun semua target sekaligus

## Konfigurasi Features

### Default Features
- `cli`: Mengaktifkan CLI binary (hanya untuk desktop)

### Optional Features
- `wasm`: WebAssembly support
- `no-default-features`: Hanya library tanpa CLI

## Verifikasi Build

### Android Library
```bash
# Cek file .so yang dihasilkan
ls -la jniLibs/*/

# Cek symbols dalam library
nm -D jniLibs/arm64-v8a/libhafiz_assistant_engine.so | grep -E "(initialize|search|get_ayah)"
```

### CLI Binary
```bash
# Cek binary executable
ls -la target/release/hafiz_assistant_engine*

# Test binary
./target/release/hafiz_assistant_engine --help
```

## Troubleshooting

### Masalah: Binary CLI muncul di Android build
**Solusi**: Pastikan menggunakan `--no-default-features` flag

### Masalah: Library tidak dihasilkan
**Solusi**: Pastikan `crate-type = ["cdylib", "rlib"]` dalam `Cargo.toml`

### Masalah: Missing symbols
**Solusi**: Cek bahwa FFI functions di-export dengan benar dalam `src/ffi/functions.rs`

## Integrasi dengan Flutter

File `.so` yang dihasilkan dapat langsung digunakan dalam Flutter project:

```dart
// Copy files ke Flutter project
cp jniLibs/* flutter_project/android/app/src/main/jniLibs/

// Gunakan dalam Dart code
import 'dart:ffi';
final DynamicLibrary lib = DynamicLibrary.open('libhafiz_assistant_engine.so');
```

## Best Practices

1. **Android**: Selalu gunakan `cargo make build-android` untuk memastikan hanya library yang dibangun
2. **Desktop**: Gunakan `cargo make build-cli` untuk development dan testing
3. **Production**: Gunakan `cargo make build-all` untuk menghasilkan semua target
4. **Testing**: Jalankan `cargo test` sebelum build untuk memastikan kode berfungsi

## Konfigurasi Cargo.toml

```toml
# Binary hanya untuk desktop
[[bin]]
name = "hafiz_assistant_engine"
path = "src/bin/main.rs"
required-features = ["cli"]  # Tidak akan dibangun tanpa feature cli

# Library untuk semua platform
[lib]
name = "hafiz_assistant_engine"
crate-type = ["cdylib", "rlib"]  # cdylib untuk FFI, rlib untuk linking
path = "src/lib.rs"

[features]
default = ["cli"]  # CLI aktif by default
cli = []           # Feature untuk CLI binary
wasm = [...]       # Feature untuk WebAssembly
```
