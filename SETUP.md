# Ourchenkoly - Setup Instructions

AI-powered meeting transcription and summarization app.

## Prerequisites

You need to install the following before building:

### 1. Node.js & pnpm
```powershell
# Check if Node.js is installed
node --version   # Should be v18+

# Install pnpm if not already installed
npm install -g pnpm
```

### 2. Rust
Download and run the installer:
- https://win.rustup.rs/x86_64

During installation, select default options. After install, restart your terminal.

```powershell
# Verify installation
rustc --version
cargo --version
```

### 3. Visual Studio Build Tools
Download and run:
- https://visualstudio.microsoft.com/visual-cpp-build-tools/

**Important**: Select "Desktop development with C++" workload during installation.

### 4. LLVM (for Whisper compilation)
Download and install LLVM 18:
- https://github.com/llvm/llvm-project/releases/download/llvmorg-18.1.8/LLVM-18.1.8-win64.exe

During installation, select "Add LLVM to the system PATH".

### 5. CMake
Download and extract to `C:\Users\<your-username>\cmake`:
- https://github.com/Kitware/CMake/releases/download/v3.31.4/cmake-3.31.4-windows-x86_64.zip

### 6. Ninja Build System
Download and extract to `C:\Users\<your-username>\ninja`:
- https://github.com/ninja-build/ninja/releases/download/v1.12.1/ninja-win.zip

---

## Quick Start

### Option A: Development Mode (slower startup, hot reload)

1. Clone the repository:
   ```powershell
   git clone https://github.com/jsn444/Scribe-Notetaker.git
   cd Scribe-Notetaker
   ```

2. Install frontend dependencies:
   ```powershell
   cd frontend
   pnpm install
   cd ..
   ```

3. Run the development server:
   ```powershell
   .\run_dev.bat
   ```

   First run takes 3-5 minutes to compile. Subsequent runs are faster.

### Option B: Release Build (fast startup, recommended for testing)

1. Build the release version:
   ```powershell
   .\build_release.bat
   ```

   This takes 5-10 minutes the first time.

2. Run the built app:
   ```powershell
   .\run_release.bat
   ```

   Or find the executable at:
   ```
   frontend\src-tauri\target\release\ourchenkoly.exe
   ```

---

## Configuration

### OpenAI API Key (for cloud transcription)

1. Open the app
2. Go to Settings (gear icon)
3. In "Transcription" section, select "OpenAI Whisper (Recommended)"
4. Enter your OpenAI API key
5. Save settings

Cloud transcription is faster than local CPU-only mode.

### Summary Generation

1. In Settings, go to "Summary" section
2. Select your preferred provider (OpenAI/Claude)
3. Enter the corresponding API key
4. Save settings

---

## Troubleshooting

### "cargo not found"
- Restart your terminal after installing Rust
- Or run: `%USERPROFILE%\.cargo\bin\cargo --version`

### "libclang not found"
- Make sure LLVM is installed and in PATH
- Set environment variable: `LIBCLANG_PATH=C:\Program Files\LLVM\bin`

### "CMake not found"
- Add CMake to PATH: `C:\Users\<your-username>\cmake\cmake-3.31.4-windows-x86_64\bin`

### Slow transcription
- Switch to OpenAI Whisper (cloud) in Settings
- Local CPU transcription is slow without GPU acceleration

### Build errors mentioning "vulkan"
- This is expected - the Windows build uses CPU-only mode
- The Cargo.toml has been configured to skip Vulkan

---

## Project Structure

```
Scribe-Notetaker/
├── frontend/           # Tauri desktop app (Rust + Next.js)
│   ├── src/           # React/TypeScript UI
│   ├── src-tauri/     # Rust backend
│   │   └── src/audio/ # Audio capture & transcription
│   └── package.json   # Node dependencies
├── run_dev.bat        # Development mode launcher
├── run_release.bat    # Release mode launcher
├── build_release.bat  # Build release executable
└── SETUP.md          # This file
```

---

## Features

- **Cloud Transcription**: OpenAI Whisper API (~$0.006/min)
- **Local Transcription**: Whisper.cpp (free, slower on CPU)
- **AI Summaries**: GPT-4, Claude, or local Ollama
- **Meeting Recording**: Captures system audio + microphone
- **Export**: JSON transcript with timestamps

---

## Support

For issues, contact Jason or file an issue on GitHub.
