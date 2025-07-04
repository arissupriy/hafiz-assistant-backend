@echo off
setlocal EnableDelayedExpansion

echo Building Hafiz Assistant Core for Android...

REM Set NDK path directly since we know the location
set "NDK_HOME=C:\Users\Dell XPS 13 7390\AppData\Local\Android\Sdk\ndk\29.0.13599879"
set "NDK_TOOLCHAIN=%NDK_HOME%\toolchains\llvm\prebuilt\windows-x86_64"

echo Using Android NDK at: %NDK_HOME%

REM Set environment variables for Rust cross-compilation
set "CC_aarch64_linux_android=%NDK_TOOLCHAIN%\bin\aarch64-linux-android21-clang.cmd"
set "AR_aarch64_linux_android=%NDK_TOOLCHAIN%\bin\llvm-ar.exe"
set "CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=%NDK_TOOLCHAIN%\bin\aarch64-linux-android21-clang.cmd"

set "CC_armv7_linux_androideabi=%NDK_TOOLCHAIN%\bin\armv7a-linux-androideabi21-clang.cmd"
set "AR_armv7_linux_androideabi=%NDK_TOOLCHAIN%\bin\llvm-ar.exe"
set "CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=%NDK_TOOLCHAIN%\bin\armv7a-linux-androideabi21-clang.cmd"

set "CC_x86_64_linux_android=%NDK_TOOLCHAIN%\bin\x86_64-linux-android21-clang.cmd"
set "AR_x86_64_linux_android=%NDK_TOOLCHAIN%\bin\llvm-ar.exe"
set "CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=%NDK_TOOLCHAIN%\bin\x86_64-linux-android21-clang.cmd"

echo Environment configured for Android cross-compilation

REM Install Android targets
echo Installing Android targets...
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android

REM Build for Android ARM64
echo.
echo Building for Android ARM64...
cargo build --release --target aarch64-linux-android

if !ERRORLEVEL! neq 0 (
    echo ERROR: Failed to build for ARM64
    pause
    exit /b !ERRORLEVEL!
)

echo ✓ ARM64 build successful

REM Build for Android ARM32
echo.
echo Building for Android ARM32...
cargo build --release --target armv7-linux-androideabi

if !ERRORLEVEL! neq 0 (
    echo ERROR: Failed to build for ARM32
    pause
    exit /b !ERRORLEVEL!
)

echo ✓ ARM32 build successful

REM Build for Android x86_64
echo.
echo Building for Android x86_64...
cargo build --release --target x86_64-linux-android

if !ERRORLEVEL! neq 0 (
    echo ERROR: Failed to build for x86_64
    pause
    exit /b !ERRORLEVEL!
)

echo ✓ x86_64 build successful

REM Copy libraries to Flutter project
set "FLUTTER_ANDROID_PATH=examples\hafiz_assistant_flutter\android\app\src\main\jniLibs"

echo.
echo Copying libraries to Flutter project...

REM Create directories
mkdir "%FLUTTER_ANDROID_PATH%\arm64-v8a" 2>nul
mkdir "%FLUTTER_ANDROID_PATH%\armeabi-v7a" 2>nul
mkdir "%FLUTTER_ANDROID_PATH%\x86_64" 2>nul

REM Copy libraries
echo Copying ARM64 library...
copy "target\aarch64-linux-android\release\libhafiz_assistant_core.so" "%FLUTTER_ANDROID_PATH%\arm64-v8a\" >nul
if !ERRORLEVEL! equ 0 (
    echo ✓ ARM64 library copied
) else (
    echo ✗ Failed to copy ARM64 library
)

echo Copying ARM32 library...
copy "target\armv7-linux-androideabi\release\libhafiz_assistant_core.so" "%FLUTTER_ANDROID_PATH%\armeabi-v7a\" >nul
if !ERRORLEVEL! equ 0 (
    echo ✓ ARM32 library copied
) else (
    echo ✗ Failed to copy ARM32 library
)

echo Copying x86_64 library...
copy "target\x86_64-linux-android\release\libhafiz_assistant_core.so" "%FLUTTER_ANDROID_PATH%\x86_64\" >nul
if !ERRORLEVEL! equ 0 (
    echo ✓ x86_64 library copied
) else (
    echo ✗ Failed to copy x86_64 library
)

echo.
echo ========================================
echo Build Complete!
echo ========================================
echo All Android libraries have been built and copied to the Flutter project.
echo.
echo Next steps:
echo 1. cd examples\hafiz_assistant_flutter
echo 2. flutter pub get
echo 3. flutter run --verbose
echo.
echo Or to build APK:
echo flutter build apk --release
echo.

pause
