# hafiz_assistant_flutter

# Hafiz Assistant Flutter

Flutter implementation of Hafiz Assistant Backend with FFI integration for Quran data access.

## Overview

This Flutter application demonstrates the integration between Flutter and the Hafiz Assistant Rust backend using Foreign Function Interface (FFI). The app provides access to Quran data including verses, surahs, search functionality, and statistics.

## Features

- **Random Ayah Display**: Get random verses from the Quran
- **Surah Browser**: Browse all 114 surahs with complete information
- **Search Functionality**: Search verses by Arabic text, translation, or transliteration
- **Statistics**: View comprehensive Quran statistics
- **Native Performance**: Direct access to Rust backend via FFI for optimal performance

## Architecture

```
Flutter App (Dart)
    ↓ FFI
Hafiz Assistant Core (Rust)
    ↓
Quran Database (JSON/SQLite)
```

## Setup

### Prerequisites

1. **Flutter SDK** (≥ 3.8.1)
2. **Rust toolchain** (for building native libraries)
3. **Android SDK** (for Android builds)
4. **Android NDK** (for cross-compilation)

### Building Native Libraries

Before running the Flutter app, you need to build the native Rust libraries:

#### For Android:

1. Install Android targets:
```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
```

2. Build libraries:
```bash
# From the root project directory
cd scripts
./build_android.sh  # On Linux/macOS
# or
build_android.bat   # On Windows
```

This will build the native libraries and copy them to the appropriate Android directories.

#### For Windows (Development):

```bash
cargo build --release
```

### Running the Flutter App

1. Navigate to the Flutter project:
```bash
cd examples/hafiz_assistant_flutter
```

2. Install dependencies:
```bash
flutter pub get
```

3. Run the app:
```bash
flutter run
```

## FFI Integration

The app uses Dart's `dart:ffi` to communicate with the Rust backend. For detailed setup and build instructions, see the project documentation.
