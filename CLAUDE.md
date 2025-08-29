# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Architecture

This is a personal about page that exists in a transition state between two frontend frameworks:

- **Legacy Svelte App**: Located in `src_svelte/` with TypeScript and Rollup build system
- **New Leptos App**: Located in `src/` using Rust/WASM with Trunk build tool

The current active branch appears to be migrating from Svelte to Leptos (Rust web framework).

### Svelte Application (Legacy)
- Entry point: `src_svelte/main.ts` 
- Main component: `src_svelte/App.svelte` with client-side routing
- Components include Header, Footer, About, Doodles, WorkHistory pages
- Uses Tailwind CSS for styling with PostCSS processing

### Leptos Application (Current)
- Entry point: `src/main.rs`
- Main component: `src/app/app.rs` 
- Modular structure with separate header/footer components
- Uses Leptos framework for reactive web components in Rust
- Compiles to WebAssembly

## Development Commands

### Leptos/Rust Development
```bash
trunk serve          # Start development server with hot reload
trunk build          # Build for production
cargo check          # Type check Rust code
```

### Legacy Svelte Development  
```bash
yarn dev             # Start development server (Rollup with watch)
yarn build           # Build for production
yarn check           # Type check with svelte-check
yarn format          # Format code with Prettier
```

## Build Systems

- **Current (Leptos)**: Uses Trunk for WASM builds, configured via `index.html` with `data-trunk` attributes
- **Legacy (Svelte)**: Uses Rollup with plugins for TypeScript, Svelte preprocessing, and Tailwind CSS

## Styling
Both applications use Tailwind CSS. The Leptos version references `input.css` while Svelte uses PostCSS processing in the build pipeline.