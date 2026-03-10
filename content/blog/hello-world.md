# Hello, World

This is the first post on my new blog. I've built this site with
[Leptos](https://leptos.dev/), a Rust web framework that compiles to
WebAssembly and runs entirely in the browser.

## Why Rust for a personal site?

Mostly because it's fun. But there are a few real reasons:

- **No JavaScript runtime** — the app compiles to a single `.wasm` binary
- **Type safety everywhere** — routes, components, and props are all checked at compile time
- **Shared toolchain** — `cargo` handles dependencies, building, and linting

The tradeoff is compile times and a smaller ecosystem, but for a personal
site that's a fine deal.

## How the blog works

Blog posts are plain markdown files that get embedded into the binary at
compile time with `include_str!()`. At runtime, `pulldown-cmark` parses
the markdown and renders it to HTML. No server, no database, no API calls.

Here's a quick example of what that looks like in Rust:

```rust
use pulldown_cmark::{Parser, html};

fn render(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}
```

## What's next

I plan to write about:

1. Building with Leptos and WASM
2. Deploying Rust apps to Netlify
3. Small programming doodles and experiments

> The best time to start a blog is ten years ago.
> The second best time is now.

Thanks for reading!
