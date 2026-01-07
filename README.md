# Trauso (TeraBox Downloader) v2

![License](https://img.shields.io/github/license/basstimam/trauso)
![Version](https://img.shields.io/github/v/release/basstimam/trauso)

A modern, fast, and robust TeraBox file downloader built with **Tauri v2** (Rust + React). Now faster, lighter, and more reliable.

![Trauso Screenshot](ss/ss1.png)

## ✨ New Features (v2)

- **🚀 Native Performance**: Built with Rust (Backend) and React (Frontend). Lightweight (~13MB) and blazing fast.
- **🛡️ Robust Fetch Engine**: 
  - Auto-fallback fetching (tries multiple API endpoints automatically).
  - Handles generic "404" or "Unknown Error" gracefully.
- **⚙️ Advanced Queue System**:
  - **Sequential Mode (Default)**: Downloads files 1-by-1 for stability.
  - **Parallel Mode**: Download up to 5 files simultaneously (Async).
  - **Auto Server Switch**: Automatically switches between Server 1 and Server 2 if one fails.
- **🖥️ Seamless Dashboard**: All-in-one interface. No more popup windows.
  - Fetch, Select, and Monitor downloads in a single view.
  - Native "Browse Folder" dialog for download location.
- **🔄 Auto Update**: Automatically checks for updates from GitHub Releases.
- **🛑 Graceful Control**: 
  - Pause/Resume/Cancel individual downloads.
  - "Clear All Queue" to remove pending/error items instantly.
  - Background process management (auto-kill aria2 on exit).

## 📥 Installation

1. Go to [Releases Page](https://github.com/basstimam/trauso/releases).
2. Download the latest `trauso_setup.exe` (Installer) or `trauso.exe` (Portable).
3. **Run and Enjoy!** No Python or external dependencies required.

## 📚 Usage guide

1. **Paste URL**: Copy your TeraBox link (supports `terabox.com`, `1024terabox.com`, `terabox.app`, etc).
2. **Fetch**: Click "Fetch". The file list will appear on the left.
3. **Select Files**: Choose files you want to download (or "Select All").
4. **Queue**: Click "Add to Queue". Files will move to the right panel.
5. **Start**: If not auto-started, click "Start" to begin downloading.

### ⚙️ Settings (Top Right Gear Icon)
- **Download Location**: Choose where files are saved.
- **Download Server**: Force Server 1 or Server 2 (Default: Server 2).
- **Download Mode**: Switch between Sequential (1-by-1) or Parallel downloads.

## 🛠️ Development (Build from Source)

### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (for Windows)

### Commands

```bash
# 1. Clone Repo
git clone https://github.com/basstimam/trauso.git
cd trauso

# 2. Install Dependencies
npm install

# 3. Run Development Mode (Hot Reload)
npm run tauri dev

# 4. Build for Production (Output: src-tauri/target/release/)
npm run tauri build
```

## 🔍 Troubleshooting

**"Failed to get info" / 404 Error:**
- Ensure the link is public and not password-protected (password support coming soon).
- Try changing your IP/VPN if Cloudflare is blocking requests.
- The app automatically tries multiple endpoints, so persistent errors usually mean the link is dead or IP banned.

**Download Speed Issues:**
- Trauso uses `aria2` with multi-connection optimization. Speed depends on TeraBox servers and your ISP.
- Try switching between "Server 1" and "Server 2" in Settings.

## 📄 License

This project is licensed under the [MIT License](LICENSE).

## ☕ Support

If you find this useful, consider supporting the development:

[![Saweria](https://img.shields.io/badge/Saweria-Support%20via%20Saweria-orange)](https://saweria.co/arumam)

---
*Disclaimer: This tool is for educational purposes only. Please respect copyright laws and TeraBox terms of service.*
