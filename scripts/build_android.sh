#!/bin/bash

# Build script for cross-platform compilation
echo "Building Hafiz Assistant Core for multiple platforms..."

# Install required targets if not already installed
echo "Installing Android targets..."
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi  
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# Build for Android ARM64 (most common)
echo "Building for Android ARM64..."
cargo build --release --target aarch64-linux-android

# Build for Android ARM32
echo "Building for Android ARM32..."
cargo build --release --target armv7-linux-androideabi

# Build for Android x86_64 (emulator)
echo "Building for Android x86_64..."
cargo build --release --target x86_64-linux-android

# Copy built libraries to Flutter Android project
FLUTTER_ANDROID_PATH="../examples/hafiz_assistant_flutter/android/app/src/main/jniLibs"

echo "Copying libraries to Flutter project..."

# Create directories if they don't exist
mkdir -p "$FLUTTER_ANDROID_PATH/arm64-v8a"
mkdir -p "$FLUTTER_ANDROID_PATH/armeabi-v7a"
mkdir -p "$FLUTTER_ANDROID_PATH/x86_64"
mkdir -p "$FLUTTER_ANDROID_PATH/x86"

# Copy ARM64 library
if [ -f "target/aarch64-linux-android/release/libhafiz_assistant_core.so" ]; then
    cp "target/aarch64-linux-android/release/libhafiz_assistant_core.so" "$FLUTTER_ANDROID_PATH/arm64-v8a/"
    echo "✓ ARM64 library copied"
fi

# Copy ARM32 library
if [ -f "target/armv7-linux-androideabi/release/libhafiz_assistant_core.so" ]; then
    cp "target/armv7-linux-androideabi/release/libhafiz_assistant_core.so" "$FLUTTER_ANDROID_PATH/armeabi-v7a/"
    echo "✓ ARM32 library copied"
fi

# Copy x86_64 library (for emulator)
if [ -f "target/x86_64-linux-android/release/libhafiz_assistant_core.so" ]; then
    cp "target/x86_64-linux-android/release/libhafiz_assistant_core.so" "$FLUTTER_ANDROID_PATH/x86_64/"
    echo "✓ x86_64 library copied"
fi

echo "Build complete!"
echo "Libraries have been copied to the Flutter Android project."
echo ""
echo "Next steps:"
echo "1. Navigate to the Flutter project: cd examples/hafiz_assistant_flutter"
echo "2. Run flutter pub get"
echo "3. Run flutter run to test the application"
