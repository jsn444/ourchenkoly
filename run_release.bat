@echo off
set "EXE_PATH=C:\Users\jason\Scribe-Notetaker\frontend\src-tauri\target\release\ourchenkoly.exe"

if exist "%EXE_PATH%" (
    echo Starting Ourchenkoly...
    start "" "%EXE_PATH%"
) else (
    echo Release build not found. Run build_release.bat first.
    echo.
    echo Expected location: %EXE_PATH%
    pause
)
