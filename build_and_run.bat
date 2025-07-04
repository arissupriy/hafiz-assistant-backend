@echo off
REM Hafiz Assistant Backend - Build Script
REM This script builds the project and runs the CLI

echo.
echo 🕌 Hafiz Assistant Backend Build Script
echo ==========================================
echo.

echo 🔨 Building project in release mode...
cargo build --release

if %ERRORLEVEL% neq 0 (
    echo.
    echo ❌ Build failed! Please check the error messages above.
    echo.
    pause
    exit /b 1
)

echo.
echo ✅ Build completed successfully!
echo.
echo 🚀 Starting CLI application...
echo.

REM Run the CLI
.\target\release\hafiz_assistant_cli.exe

echo.
echo 👋 CLI closed. Press any key to exit...
pause > nul
