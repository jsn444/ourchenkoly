@echo off
REM Set up paths
set "PATH=%USERPROFILE%\.cargo\bin;C:\Program Files\LLVM\bin;C:\Users\jason\ninja;C:\Users\jason\cmake\cmake-3.31.4-windows-x86_64\bin;%PATH%"
set "LIBCLANG_PATH=C:\Program Files\LLVM\bin"
set "CMAKE_GENERATOR=Ninja"

REM Run from VS Developer environment to get MSVC in PATH
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul 2>&1

cd /d C:\Users\jason\Scribe-Notetaker\frontend
pnpm run tauri:dev:cpu
