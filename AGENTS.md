# AGENTS.md - Trauso Development Guide

## Build Commands

### Python (Legacy CLI/GUI)
```bash
# Install dependencies
pip install -r requirements.txt

# Build standalone executable (menu-based)
python build.py

# Run CLI version
python terabox_cli.py

# Run GUI version
python terabox_gui.py
```

### Tauri (Modern React/Rust)
```bash
cd trauso/

# Install dependencies (npm or bun)
npm install  # or bun install

# Development server
npm run dev  # or bun run dev

# Build for production
npm run build  # or bun run build

# Preview production build
npm run preview

# Build desktop app (Tauri)
npm run tauri build
```

### Rust Backend
```bash
cd trauso/src-tauri/

# Run tests
cargo test

# Check code
cargo check

# Build release
cargo build --release
```

## Testing
**No formal test framework configured** in this project. When adding tests:
- For Python: Consider adding `pytest`
- For TypeScript: Consider adding `vitest` or `jest`
- For Rust: Use `cargo test` (already available)

## Code Style Guidelines

### Python Files (CLI/GUI)

#### Import Organization
- Follow PEP 8: Standard Library → Third-party → Local modules
- Group imports with blank lines between groups
- Type hints: Import from `typing` module

```python
import os
import sys
from pathlib import Path

import requests
from rich.console import Console

from terabox1 import TeraboxFile, TeraboxLink
```

#### Naming Conventions
- Classes: `PascalCase` (e.g., `TeraboxDownloader`, `TeraboxGUI`)
- Functions/Methods: `snake_case` (e.g., `extract_shorturl`, `get_download_link`)
- Variables: `snake_case` (e.g., `short_url`, `download_dir`)
- Constants: `UPPER_CASE` (e.g., `BASE_URLS`, `WORKERS_ENDPOINTS`)
- Private/Internal: Prefix with `_` (e.g., `_create_session`, `_file_cache`)

#### Type Hinting
- **Required** on all function arguments and return values
- Use `Optional[T]` for nullable types
- Use `Dict[str, Any]` for flexible dictionaries

```python
def get_info(shorturl: str, cookies: Optional[Dict[str, str]] = None) -> Optional[Dict[str, Any]]:
    pass
```

#### Docstrings
- Use triple double quotes `"""`
- Google/NumPy style with `Args:` and `Returns:` sections
- Mix of Indonesian and English is acceptable

```python
def extract_download_url(self) -> Optional[str]:
    """
    Extract direct download URL from TeraBox page.

    Args:
        None

    Returns:
        Direct download URL if found, None otherwise.
    """
```

#### Error Handling
- Use `try...except Exception as e` for risky operations (I/O, Network)
- Log errors using `logger.error()`
- Display to users via `rich.console` (CLI) or `messagebox` (GUI)

```python
try:
    response = requests.get(url, timeout=15)
    response.raise_for_status()
except Exception as e:
    self.logger.error(f"Download failed: {str(e)}")
    console.print(f"[red]Error: {str(e)}[/]")
```

#### Logging
- Use standard library `logging`
- Instance: `logger = logging.getLogger(__name__)`
- Logs saved to `/logs` folder with daily rotation (e.g., `terabox_20260107.log`)

```python
import logging
logger = logging.getLogger(__name__)
logger.info("Starting download...")
logger.error(f"Error: {str(e)}")
```

#### Formatting
- Indentation: 4 spaces (NOT tabs)
- Two blank lines between top-level functions/classes
- Line length: Standard PEP 8 (~79-99 chars), longer acceptable for UI/API parameters

### TypeScript/React (Tauri Frontend)

#### Import Style
- Group imports: React → Third-party → Local components
- Named imports preferred for better tree-shaking

```typescript
import { useState, useEffect } from 'react'
import { Button } from '@/components/ui/button'
import { DownloadProgress } from './DownloadProgress'
```

#### Component Style
- Use functional components with hooks
- Props should be typed with interfaces
- Use TypeScript strict mode

#### CSS/Tailwind
- Use Tailwind CSS utility classes
- Follow component-based structure
- Use `cn()` helper from `clsx` and `tailwind-merge` for conditional classes

### Rust (Tauri Backend)

#### Naming
- Functions/Variables: `snake_case`
- Types/Structs: `PascalCase`
- Constants: `UPPER_CASE`

#### Error Handling
- Use `Result<T, E>` for fallible operations
- Use `?` operator for early returns
- Avoid `unwrap()` in production code

## Project Architecture

This is a **hybrid project** with two implementations:

### Legacy (Python)
- `terabox_cli.py` - CLI interface using Rich for TUI
- `terabox_gui.py` - GUI interface using Tkinter + Sun Valley theme
- `workers.py` - API workers with strong type hints
- `terabox1.py` - Low-level TeraBox API abstraction
- `build.py` - Build script for standalone executable distribution

### Modern (Tauri)
- `trauso/src/main.tsx` - React frontend entry point
- `trauso/src-tauri/src/lib.rs` - Rust backend with Tauri commands
- `trauso/src-tauri/src/terabox/` - TeraBox API module (get_info, get_download_link)
- `trauso/src-tauri/src/aria2/` - aria2 RPC client for download management
- `trauso/src/lib/api.ts` - Frontend API wrapper for Tauri commands
- `trauso/src/lib/types.ts` - TypeScript types for API
- `trauso/src/components/file-list.tsx` - File browser with tree view
- `trauso/src/components/download-progress.tsx` - Download progress with controls
- Uses Vite for bundling, Tailwind CSS v3 for styling
- Radix UI + Shadcn UI components for UI consistency

#### Tauri Commands Available
```typescript
get_terabox_info(url: string)        // Get file info from TeraBox URL
get_download_link(params)            // Get direct download link
extract_shorturl(url: string)        // Extract shorturl from URL
start_aria2()                        // Start aria2 daemon
add_download(url, dir?, filename?)   // Add download to aria2
get_download_status(gid: string)     // Get download progress
pause_download(gid: string)          // Pause download
resume_download(gid: string)         // Resume download
cancel_download(gid: string)         // Cancel download
get_all_downloads()                  // List all downloads
```

### Shared Dependencies
- `aria2/` - Binary `aria2c.exe` for download management
- `aria2.conf` - Aria2 configuration

## Development Workflow

1. **For Python changes**: Edit files in root directory, test with `python terabox_cli.py` or `python terabox_gui.py`
2. **For Tauri changes**:
   - Frontend: Work in `trauso/src/`, test with `npm run dev`
   - Backend: Work in `trauso/src-tauri/src/`, test with `cargo test`
3. **Before committing**: Ensure type hints are complete and logging is added for new features

## Notes
- No formal linting/formatting tools configured (no ESLint, Prettier, Black, etc.)
- Aria2 binary must be present in `aria2/` folder for download functionality
- Application logs stored in `logs/` folder with daily rotation
- Settings stored in `config/settings.json` (Python) or app config (Tauri)
