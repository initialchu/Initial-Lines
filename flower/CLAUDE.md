# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
你精通tauri桌面应用开发，将尽最大的努力来帮助我开发这个软件，而且将使用尽量符合企业标准的方法。
## Project

**初笺** — a Tauri v2 desktop markdown editor. Vue 3 + TypeScript frontend, Rust backend.

## Commands

```bash
npm run tauri dev    # Full desktop app in dev mode
npm run tauri build  # Production desktop bundle
npm run dev          # Vite dev server only (port 1420)
npm run build        # Type-check + build frontend only
```

## Architecture

Frontend (`src/`) calls Rust commands via `invoke("name", { args })` from `@tauri-apps/api/core`.
Rust commands are `#[tauri::command]` functions and **must** be registered in `generate_handler![]` inside `src-tauri/src/lib.rs:run()`.
New native capabilities (fs, clipboard, shortcuts) need permissions added to `src-tauri/capabilities/default.json`.
