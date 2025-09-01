# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Architecture

This is a personal about page built with the Leptos web framework (Rust/WASM).

### Application Structure
- Entry point: `src/main.rs`
- Main component: `src/app/app.rs` 
- Modular component structure:
  - Header: `src/app/header.rs`
  - Footer: `src/app/footer.rs`
- Uses Leptos framework for reactive web components in Rust
- Compiles to WebAssembly for browser execution

## Development Commands

```bash
trunk serve          # Start development server with hot reload
trunk build          # Build for production
cargo check          # Type check Rust code
cargo clippy         # Lint Rust code
cargo fmt            # Format Rust code
```

## Build System

Uses Trunk for WASM builds, configured via:
- `Trunk.toml` for build configuration
- `index.html` with `data-trunk` attributes for asset processing
- `rust-toolchain.toml` for Rust version specification

## Styling

Uses Tailwind CSS with:
- Input file: `input.css` 
- PostCSS configuration via `postcss.config.js`

## Deployment

Configured for Netlify deployment via:
- `netlify.toml` configuration
- `netlify.sh` build script
