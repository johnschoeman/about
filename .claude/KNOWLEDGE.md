# Knowledge

Stable knowledge for this repository. Read at session start. Organized by topic, not chronologically.

---

## Decisions

### getrandom wasm_js configuration
The `wasm_js` feature is set in `Cargo.toml` on the `getrandom` dependency. The old `.cargo/config.toml` approach (`--cfg getrandom_backend="wasm_js"` rustflag) was deprecated in getrandom 0.3+ and removed. No config.toml is needed.

### rand 0.10 trait import
In rand 0.10, `random_range()` moved from the `Rng` trait to `RngExt`. Import `use rand::RngExt;` (not `use rand::Rng;`). The `rand::rng()` function and `.random_range()` method signatures are unchanged.

## Learnings

### NixOS + rustup toolchain
On this NixOS system, rustup-installed toolchain binaries can break when nix garbage-collects the glibc they were linked against. Workaround: use `nix-shell -p cargo rustc` for a working toolchain, or run `rustup update nightly` to get binaries linked against the current glibc.

### Tailwind CSS version
The project uses Tailwind CSS v4 via trunk's `data-trunk rel="tailwind-css"` directive. Netlify gets v4 via `brew install tailwindcss`. The nixpkgs `tailwindcss` package is v3, which is incompatible — it fails with "Can't resolve 'tailwindcss'". Local dev needs the v4 standalone CLI binary on PATH.

### trunk build requires working cargo
`trunk build` shells out to `cargo`, so the same NixOS glibc issue applies. Run trunk inside `nix-shell -p cargo rustc` if the rustup toolchain is broken.
