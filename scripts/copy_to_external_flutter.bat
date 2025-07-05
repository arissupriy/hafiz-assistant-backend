@echo off
setlocal EnableDelayedExpansion

echo ========================================
echo Hafiz Assistant - Copy to External Flutter Project
echo ========================================

set "SOURCE_DIR=%cd%"
set "TARGET_DIR=C:\Project\hafiz_assistant\examples\hafiz_assistant_flutter"

echo Source: %SOURCE_DIR%
echo Target: %TARGET_DIR%
echo.

REM Check if target directory exists
if not exist "%TARGET_DIR%" (
    echo ERROR: Target Flutter project not found at %TARGET_DIR%
    echo Please check the path and try again.
    pause
    exit /b 1
)

echo Creating target directories...

REM Create jniLibs directories
mkdir "%TARGET_DIR%\android\app\src\main\jniLibs\arm64-v8a" 2>nul
mkdir "%TARGET_DIR%\android\app\src\main\jniLibs\armeabi-v7a" 2>nul
mkdir "%TARGET_DIR%\android\app\src\main\jniLibs\x86_64" 2>nul

echo Copying Android libraries...

REM Copy ARM64 library
echo - ARM64 library...
copy "%SOURCE_DIR%\target\aarch64-linux-android\release\libhafiz_assistant_core.so" "%TARGET_DIR%\android\app\src\main\jniLibs\arm64-v8a\libhafiz_assistant_core.so" >nul
if !errorlevel! equ 0 (
    echo   ✓ ARM64 library copied
) else (
    echo   ✗ Failed to copy ARM64 library
)

REM Copy ARM32 library  
echo - ARM32 library...
copy "%SOURCE_DIR%\target\armv7-linux-androideabi\release\libhafiz_assistant_core.so" "%TARGET_DIR%\android\app\src\main\jniLibs\armeabi-v7a\libhafiz_assistant_core.so" >nul
if !errorlevel! equ 0 (
    echo   ✓ ARM32 library copied
) else (
    echo   ✗ Failed to copy ARM32 library
)

REM Copy x86_64 library
echo - x86_64 library...
copy "%SOURCE_DIR%\target\x86_64-linux-android\release\libhafiz_assistant_core.so" "%TARGET_DIR%\android\app\src\main\jniLibs\x86_64\libhafiz_assistant_core.so" >nul
if !errorlevel! equ 0 (
    echo   ✓ x86_64 library copied
) else (
    echo   ✗ Failed to copy x86_64 library
)

echo.
echo Copying Dart bindings and example code...

REM Copy lib directory (Dart code)
xcopy "%SOURCE_DIR%\examples\hafiz_assistant_flutter\lib\*" "%TARGET_DIR%\lib\" /S /Y >nul
if !errorlevel! equ 0 (
    echo   ✓ Dart bindings copied
) else (
    echo   ✗ Failed to copy Dart bindings
)

REM Copy pubspec.yaml if it doesn't exist
if not exist "%TARGET_DIR%\pubspec.yaml" (
    echo - Copying pubspec.yaml...
    copy "%SOURCE_DIR%\examples\hafiz_assistant_flutter\pubspec.yaml" "%TARGET_DIR%\pubspec.yaml" >nul
    if !errorlevel! equ 0 (
        echo   ✓ pubspec.yaml copied
    ) else (
        echo   ✗ Failed to copy pubspec.yaml
    )
) else (
    echo   - pubspec.yaml already exists (skipped)
)

REM Copy ffigen.yaml if it doesn't exist  
if not exist "%TARGET_DIR%\ffigen.yaml" (
    echo - Copying ffigen.yaml...
    copy "%SOURCE_DIR%\examples\hafiz_assistant_flutter\ffigen.yaml" "%TARGET_DIR%\ffigen.yaml" >nul
    if !errorlevel! equ 0 (
        echo   ✓ ffigen.yaml copied
    ) else (
        echo   ✗ Failed to copy ffigen.yaml
    )
) else (
    echo   - ffigen.yaml already exists (skipped)
)

REM Copy integration guide
echo - Copying integration guide...
copy "%SOURCE_DIR%\ANDROID_INTEGRATION_GUIDE.md" "%TARGET_DIR%\INTEGRATION_GUIDE.md" >nul
if !errorlevel! equ 0 (
    echo   ✓ Integration guide copied
) else (
    echo   ✗ Failed to copy integration guide
)

echo.
echo ========================================
echo Copy Complete!
echo ========================================
echo.
echo Next steps:
echo 1. cd /d "%TARGET_DIR%"
echo 2. flutter clean
echo 3. flutter pub get
echo 4. flutter pub run ffigen
echo 5. flutter run --verbose
echo.
echo Or to build APK:
echo flutter build apk --release
echo.
echo For detailed instructions, see: INTEGRATION_GUIDE.md
echo.
pause
