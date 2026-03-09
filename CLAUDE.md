# CLAUDE.md

Personal portfolio/about site for John Schoeman. Built with the Leptos web framework (Rust/WASM), client-side rendered, styled with Tailwind CSS v4, and deployed to Netlify.

## Development Commands

```bash
trunk serve           # Start development server with hot reload
trunk build           # Development build
trunk build --release # Production build
cargo check           # Type check
cargo clippy          # Lint
cargo fmt             # Format
```

## Repository Structure

```
src/
  main.rs                          # Entry point, mounts App to body
  app/
    app.rs                         # Root component with Router and routes
    header.rs                      # Navigation bar (desktop + mobile)
    footer.rs                      # Social links footer
    mod.rs
    pages/
      home/
        home.rs                    # Landing page
        ortho_board_doodle.rs      # Interactive cell automaton doodle on home
      about.rs                     # About page
      doodles/
        doodles.rs                 # Index of all doodles (internal + external)
        increment_doodle.rs        # Increment doodle (hosted in this app)
      work_history/
        work_history.rs            # Work history page
        work_history_header.rs
        work_summary.rs
        full_timeline.rs
        detailed_experience.rs
index.html          # Trunk entry HTML (data-trunk directives)
input.css           # Tailwind entry point (@import "tailwindcss")
postcss.config.js   # PostCSS config (uses @tailwindcss/postcss)
netlify.toml        # Netlify build config
netlify.sh          # Netlify build script (installs trunk + tailwindcss, runs trunk build)
rust-toolchain.toml # Nightly Rust toolchain, wasm32-unknown-unknown target
.cargo/config.toml  # Sets getrandom_backend="wasm_js" rustflag
public/             # Static assets (favicon)
```

## Architecture

- **Framework:** Leptos 0.8 with client-side rendering (`csr` feature). Components use `#[component]` macro and `view!` macro for RSX.
- **Routing:** `leptos_router` with `<Router>`, `<Routes>`, `<Route path=path!("/...")>` pattern. Routes: `/`, `/about`, `/doodles`, `/doodles/increment`, `/work_history`.
- **Randomness:** Uses `rand` + `getrandom` with `wasm_js` feature for browser-compatible RNG (configured via `.cargo/config.toml` rustflag).
- **Build:** Trunk compiles Rust to WASM, processes `data-trunk` link elements in `index.html` for assets and Tailwind CSS.
- **Styling:** Tailwind CSS v4 via Trunk's built-in `tailwind-css` integration. All styling is utility classes in `view!` templates.
- **Deploy:** Netlify with `netlify.sh` build script. Uses Homebrew to install trunk and tailwindcss on the Netlify build image.

## Key Configurations

| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust edition 2024, Leptos CSR, rand with wasm_js getrandom |
| `rust-toolchain.toml` | Nightly channel, rust-analyzer + rustfmt, wasm32 target |
| `.cargo/config.toml` | `getrandom_backend="wasm_js"` rustflag for WASM RNG |
| `index.html` | Trunk directives: rust build, favicon copy, tailwind-css processing |
| `postcss.config.js` | `@tailwindcss/postcss` plugin |

## Git Workflow

User manages all commits. You remind and suggest, never commit.

**After significant work:**
1. Run `/update-session-log` to capture session context
2. Run `git status` to see what's actually uncommitted
3. Write commit message to `commit-msg.txt`
4. Remind: "You may want to commit these changes"

**Commit message format:**
```
Concise summary (one line)

Why:

[Brief motivation - what problem this solves]

This commit:

- [Bullet points of actual changes]
- [Only include uncommitted changes, not already-committed work]
```

**Never:** Run git commands that modify history.

## Specs

`.claude/specs.toml` tracks acceptance criteria with pass/fail status.
When verifying, run checks inside `devenv shell` (prefix: `devenv shell --`).
Update status and date after checking. Add new specs when adding features.

## Conventions

- Rust nightly, edition 2024
- Module structure uses `mod.rs` files for re-exports
- Component filenames use snake_case matching the component name
- Tailwind utility classes applied directly in `view!` macro templates
- Links and navigation items defined as inline data within components
