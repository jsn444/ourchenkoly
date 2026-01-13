@echo off
echo ========================================
echo Building Ourchenkoly Release Version
echo ========================================
echo.

set "PATH=%USERPROFILE%\.cargo\bin;C:\Program Files\LLVM\bin;C:\Users\jason\ninja;C:\Users\jason\cmake\cmake-3.31.4-windows-x86_64\bin;%PATH%"
set "LIBCLANG_PATH=C:\Program Files\LLVM\bin"
set "CMAKE_GENERATOR=Ninja"

call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1

cd /d C:\Users\jason\Scribe-Notetaker\frontend

echo [1/2] Building Next.js frontend...
call pnpm run build
if errorlevel 1 (
    echo ERROR: Frontend build failed
    pause
    exit /b 1
)

echo.
echo [2/2] Building Tauri release (this takes a few minutes first time)...
call pnpm run tauri:build:cpu
if errorlevel 1 (
    echo ERROR: Tauri build failed
    pause
    exit /b 1
)

echo.
echo ========================================
echo Build complete!
echo.
echo Installer location:
echo   frontend\src-tauri\target\release\bundle\nsis\
echo.
echo Portable executable:
echo   frontend\src-tauri\target\release\ourchenkoly.exe
echo ========================================
pause
