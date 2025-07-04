@echo off
REM Build script for cross-platform compilation on Windows with Android NDK

echo Building Hafiz Assistant Core for multiple platforms...

REM Detect Android NDK installation using ANDROID_HOME
set NDK_FOUND=0
set "ANDROID_NDK_HOME="

echo Checking for Android NDK using ANDROID_HOME environment variable...
if defined ANDROID_HOME (
    echo ANDROID_HOME found at: %ANDROID_HOME%
    
    REM Check for NDK in ANDROID_HOME/ndk directory
    for /d %%i in ("%ANDROID_HOME%\ndk\*") do (
        if exist "%%i\toolchains\llvm\prebuilt\windows-x86_64\bin\aarch64-linux-android21-clang.cmd" (
            set "ANDROID_NDK_HOME=%%i"
            set NDK_FOUND=1
            goto :ndk_found
        )
    )
    
    REM Check for NDK bundle (older installations)
    if exist "%ANDROID_HOME%\ndk-bundle\toolchains\llvm\prebuilt\windows-x86_64\bin\aarch64-linux-android21-clang.cmd" (
        set "ANDROID_NDK_HOME=%ANDROID_HOME%\ndk-bundle"
        set NDK_FOUND=1
        goto :ndk_found
    )
) else (
    echo ANDROID_HOME not set, checking common locations...
    
    REM Check common NDK locations
    for /d %%i in ("%LOCALAPPDATA%\Android\Sdk\ndk\*") do (
        if exist "%%i\toolchains\llvm\prebuilt\windows-x86_64\bin\aarch64-linux-android21-clang.cmd" (
            set "ANDROID_NDK_HOME=%%i"
            set NDK_FOUND=1
            goto :ndk_found
        )
    )
)

:ndk_found
if %NDK_FOUND%==0 (
    echo ERROR: Android NDK not found!
    echo Please ensure Android NDK is installed. You can install it via Android Studio:
    echo 1. Open Android Studio
    echo 2. Go to Tools ^> SDK Manager
    echo 3. Switch to SDK Tools tab
    echo 4. Check NDK (Side by side) and install
    echo.
    echo Or set ANDROID_HOME to point to your Android SDK directory
    echo Current ANDROID_HOME: %ANDROID_HOME%
    pause
    exit /b 1
)

echo Found Android NDK at: %ANDROID_NDK_HOME%

REM Set up environment for cross-compilation
set "NDK_TOOLCHAIN=%ANDROID_NDK_HOME%\toolchains\llvm\prebuilt\windows-x86_64"
set "PATH=%NDK_TOOLCHAIN%\bin;%PATH%"

REM Set environment variables for Rust
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

REM Install required targets if not already installed
echo Installing Android targets...
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi  
rustup target add x86_64-linux-android
rustup target add i686-linux-android

REM Build for Android ARM64 (most common)
echo Building for Android ARM64...
cargo build --release --target aarch64-linux-android

if %ERRORLEVEL% neq 0 (
    echo ERROR: Failed to build for ARM64
    pause
    exit /b %ERRORLEVEL%
)

REM Build for Android ARM32
echo Building for Android ARM32...
cargo build --release --target armv7-linux-androideabi

if %ERRORLEVEL% neq 0 (
    echo ERROR: Failed to build for ARM32
    pause
    exit /b %ERRORLEVEL%
)

REM Build for Android x86_64 (emulator)
echo Building for Android x86_64...
cargo build --release --target x86_64-linux-android

if %ERRORLEVEL% neq 0 (
    echo ERROR: Failed to build for x86_64
    pause
    exit /b %ERRORLEVEL%
)

REM Set Flutter Android path
set FLUTTER_ANDROID_PATH=..\examples\hafiz_assistant_flutter\android\app\src\main\jniLibs

echo Copying libraries to Flutter project...

REM Create directories if they don't exist
mkdir "%FLUTTER_ANDROID_PATH%\arm64-v8a" 2>nul
mkdir "%FLUTTER_ANDROID_PATH%\armeabi-v7a" 2>nul
mkdir "%FLUTTER_ANDROID_PATH%\x86_64" 2>nul
mkdir "%FLUTTER_ANDROID_PATH%\x86" 2>nul

REM Copy ARM64 library
if exist "target\aarch64-linux-android\release\libhafiz_assistant_core.so" (
    copy "target\aarch64-linux-android\release\libhafiz_assistant_core.so" "%FLUTTER_ANDROID_PATH%\arm64-v8a\"
    echo ✓ ARM64 library copied
) else (
    echo ✗ ARM64 library not found
)

REM Copy ARM32 library
if exist "target\armv7-linux-androideabi\release\libhafiz_assistant_core.so" (
    copy "target\armv7-linux-androideabi\release\libhafiz_assistant_core.so" "%FLUTTER_ANDROID_PATH%\armeabi-v7a\"
    echo ✓ ARM32 library copied
) else (
    echo ✗ ARM32 library not found
)

REM Copy x86_64 library (for emulator)
if exist "target\x86_64-linux-android\release\libhafiz_assistant_core.so" (
    copy "target\x86_64-linux-android\release\libhafiz_assistant_core.so" "%FLUTTER_ANDROID_PATH%\x86_64\"
    echo ✓ x86_64 library copied
) else (
    echo ✗ x86_64 library not found
)

echo.
echo Build complete!
echo Libraries have been copied to the Flutter Android project.
echo.
echo Android NDK used: %ANDROID_NDK_HOME%
echo.
echo Next steps:
echo 1. Navigate to the Flutter project: cd examples\hafiz_assistant_flutter
echo 2. Run flutter pub get
echo 3. Run flutter run to test the application

pause
