# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Serve with hot-reload (web, default)
dx serve

# Serve for desktop
dx serve --platform desktop

# Build for release
dx build --release

# Check/lint
cargo clippy
```

Tailwind CSS is handled automatically by Dioxus 0.7 — no separate Tailwind CLI step needed. It picks up `tailwind.css` next to `Cargo.toml` automatically on `dx serve`.

## Architecture

Single-page portfolio built with **Dioxus 0.7** (Rust WASM framework), targeting web by default.

**Entry point:** `src/main.rs` defines the `Route` enum, launches the app, and renders the `Home` page — a vertical stack of section components.

**Routing:** Uses `Navbar` as a layout wrapper (via `#[layout(Navbar)]`) for all routes. Only one route exists: `/` → `Home`.

**Components** (`src/components/`): Each portfolio section is its own component module — `hero`, `about`, `skills`, `experience`, `education`, `footer`, and `navbar`. All are re-exported from `src/components/mod.rs`.

**Styling:** Tailwind CSS via `assets/tailwind.css`. Google Fonts (Chakra Petch, Barlow, Share Tech Mono) injected via `document::Link` in `App`.

**Assets:** Referenced via the `asset!()` macro (e.g., `asset!("/assets/favicon.ico")`).

## Dioxus 0.7 API Notes

- No `cx`, `Scope`, or `use_state` — these are removed in 0.7
- State: `use_signal(|| value)`, read with `signal()` or `.read()`, write with `.write()`
- Memoization: `use_memo(move || ...)`
- Async: `use_resource(move || async move { ... })`
- Components: `#[component]` macro, props must be owned (`String` not `&str`), must implement `PartialEq + Clone`
- Stylesheets: `document::Stylesheet { href: asset!(...) }` or `document::Link`
- The `clippy.toml` forbids holding `GenerationalRef`/`GenerationalRefMut`/`WriteLock` across `.await` points
