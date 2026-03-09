# Knowledge

Stable knowledge for this repository. Read at session start. Organized by topic, not chronologically.

---

## Decisions

### Local dev environment: devenv
Local development uses devenv (not rustup) for reproducible tooling. `devenv.nix` provides Rust nightly + wasm32 target (via rust-overlay), trunk, and tailwindcss v4. direnv auto-activates the shell on `cd`. Netlify CI is separate — it uses brew in `netlify.sh`.

### getrandom wasm_js configuration
The `wasm_js` feature is set in `Cargo.toml` on the `getrandom` dependency. The old `.cargo/config.toml` approach (`--cfg getrandom_backend="wasm_js"` rustflag) was deprecated in getrandom 0.3+ and removed. No config.toml is needed.

### rand 0.10 trait import
In rand 0.10, `random_range()` moved from the `Rng` trait to `RngExt`. Import `use rand::RngExt;` (not `use rand::Rng;`). The `rand::rng()` function and `.random_range()` method signatures are unchanged.

## Learnings

### devenv `languages.rust.channel` requires rust-overlay input
Using `languages.rust.channel = "nightly"` in devenv requires adding the rust-overlay input: `devenv inputs add rust-overlay github:oxalica/rust-overlay --follows nixpkgs`. Without it, `devenv shell` fails with a clear error message telling you to add it.

### devenv direnv integration: use `eval "$(devenv direnvrc)"`
The modern `.envrc` approach (devenv v1.4+) is `eval "$(devenv direnvrc)"`. The older `source_url` approach pinning a GitHub commit hash breaks when upstream force-pushes or reorganizes — the URL 404s. The `eval` approach loads from the local devenv install and is self-updating.

### Tailwind CSS version
The project uses Tailwind CSS v4 via trunk's `data-trunk rel="tailwind-css"` directive. Locally, devenv provides `pkgs.tailwindcss_4`. Netlify gets v4 via `brew install tailwindcss`. The nixpkgs `tailwindcss` (no suffix) package is v3, which is incompatible.

### NixOS + rustup toolchain
On NixOS, rustup-installed toolchain binaries can break when nix garbage-collects the glibc they were linked against. The devenv setup avoids this entirely by providing Rust via rust-overlay (nix-native). If not using devenv, workaround: `nix-shell -p cargo rustc` or `rustup update nightly`.
