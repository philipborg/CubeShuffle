# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

CubeShuffle is a card-game cube-shuffling utility. It implements "Distribution Shuffle": cards are grouped into named piles, each pile has a card count and a "randomness" (0.0–1.0) odds value, and the core algorithm deals piles into packs of a given size, optionally randomizing a fraction of each pile's cards across packs. It's game-agnostic (originally aimed at MTG cube drafting). See `README.adoc` and `docs/distribution_shuffle.adoc` for the user-facing explanation of the algorithm/workflow.

## Workspace layout

Cargo workspace (`Cargo.toml`) with these members, plus a Cordova wrapper that is not part of the Cargo workspace:

- `cube_shuffle-core` — the shuffling algorithm and data types (`Pile`, `Pack`, `ShufflingErrors`, `shuffle()`). Pure logic, no I/O. All other crates depend on this.
- `cube_shuffle-cli` — `clap`-based CLI binary. Parses `name=cards:randomness` pile arguments, calls `shuffle()`, formats output as Debug/PrettyDebug/JSON/YAML.
- `cube_shuffle-wasm` — Yew (`csr`) web frontend, built with `trunk`. Uses git-main `yew` (not crates.io) — see `cube_shuffle-wasm/Cargo.toml`. Styled with Bulma + bulma-tooltip, pulled in as git submodules under `cube_shuffle-wasm/bulma*`. `index.html` + `style.sass` are the trunk entry points.
- `src-tauri` — Tauri desktop shell that wraps the built wasm frontend. Its `beforeBuildCommand` runs `trunk build` for `cube_shuffle-wasm` into `src-tauri/websrc`, which Tauri then bundles. There is no separate desktop UI — the desktop app is the wasm/Yew app running inside a Tauri webview.
- `cube_shuffle-cordova` — Cordova wrapper for Android; its `www` dir is populated from the same `trunk`-built wasm output in CI (not built locally by cargo).

Because desktop and Android both embed the wasm build, UI changes are made once in `cube_shuffle-wasm` and propagate to web, desktop, and Android.

## Common commands

Run all of these from the repo root (workspace-wide) unless noted.

```sh
# Build workspace (core, cli, wasm-as-lib-target, tauri)
cargo build --workspace

# Run all tests (core has proptest-based property tests)
cargo test --workspace

# Run a single test
cargo test -p cube_shuffle-core shuffled_cards

# Lint (CI runs this with -D warnings, i.e. warnings fail the build)
cargo clippy --all-features --workspace --all-targets -- -D warnings

# Format check / apply
cargo fmt --all --check
cargo fmt --all

# Run the CLI directly
cargo run -p cube_shuffle-cli -- distribution -p "White=50:0.1" -p "Blue=50:0.2" -s 15
```

Web/desktop/mobile builds require tools not installed by `cargo build` alone:

```sh
# Web (wasm) build — requires `trunk` and target wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cd cube_shuffle-wasm && trunk build --release --dist ../dist_wasm

# Desktop (Tauri) build — requires the tauri-cli (pinned version in .github/workflows/build.yml)
cargo install tauri-cli --locked --version <TAURI_CLI_VERSION from build.yml>
cd src-tauri && cargo tauri build
```

Git submodules (`cube_shuffle-wasm/bulma`, `cube_shuffle-wasm/bulma-tooltip`) must be checked out for the wasm frontend to build: `git submodule update --init --recursive`.

## CI reference

`.github/workflows/build.yml` is the authoritative build/lint/test matrix — mirror it when unsure how a change will be validated: `cargo build --release --locked`, `cargo clippy --all-features --workspace --all-targets -- -D warnings`, `cargo test --release --locked --no-fail-fast`, `cargo fmt --all --check`, run on the native (non-wasm) targets; a separate `wasm-build` job runs `trunk build` for the frontend; `cli-build` cross-compiles the CLI to a large target matrix; `cordova-build`/`apk-generate` produce the Android bundle/APK from the wasm output.