# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`mlbtg` is Anthony's fork of [mlb-rs/mlbt](https://github.com/mlb-rs/mlbt) — a ratatui terminal UI for MLB's Stats API. The fork layers visual-accessibility additions (color themes, Nerd Font glyphs, team colors, weather) on top of upstream. **All additions are off by default** — without config changes, behavior matches upstream.

The binary is named `mlbtg` but the config app name is still `mlbt` (config path: `~/.config/mlbt/mlbt.toml`).

## Workspace layout

Cargo workspace with two members:

- `.` — main binary `mlbtg` (`src/`)
- `api/` — `mlbt-api` crate that wraps the MLB Stats API

`src/` is split by concern:
- `main.rs` — tokio runtime, sets up three tasks (input handler, `NetworkWorker`, `PeriodicRefresher`) communicating via mpsc channels around an `Arc<Mutex<App>>`
- `app.rs` — top-level `App` struct
- `state/` — per-tab state (schedule, gameday, stats, standings, team_page, player_profile), plus `network.rs` (worker), `refresher.rs` (periodic polling), `messages.rs` (channel message types), `cache.rs`, `settings_editor.rs`
- `components/` — data → display transforms, one module per UI surface; shared helpers in `components/util.rs`
- `ui/` — ratatui rendering for each surface
- `draw.rs` — top-level draw dispatch
- `keys.rs` — key bindings → state mutations / network requests
- `theme.rs`, `symbols.rs`, `config.rs` — color tiers, Nerd Font glyphs, TOML config loader

Architecture is event-driven: `UiEvent` (key/resize) → state mutations → `NetworkRequest` → `NetworkResponse` → state update → redraw. Each tab owns its own date independent of the scoreboard.

## Common commands

```sh
cargo build --release           # release binary at target/release/mlbtg
cargo run                       # run the TUI
cargo run -- --version          # print version and exit (no TUI)

cargo fmt --all -- --check      # CI format check (must pass)
cargo clippy --all-features --all-targets --workspace --locked -- --deny warnings  # CI lint (must pass, zero warnings)
cargo test --all-features --all-targets --workspace --locked   # CI tests

cargo test -p mlbt-api          # api crate only
cargo test -p mlbt-api <name>   # single test by name substring
```

CI (`.github/workflows/ci.yml`) runs fmt + clippy + tests + docker build on every PR. **Always run `cargo fmt` and `cargo clippy --workspace -- --deny warnings` before pushing** — both must be clean or CI fails.

`rust-toolchain.toml` pins the toolchain; `rustup show` will install it.

## API crate testing

`api/tests/client.rs` uses `mockito` against JSON fixtures in `api/tests/responses/`. When changing or adding an endpoint:

1. Drop a real API response into `api/tests/responses/` as `.json`
2. Reference it from a test in `api/tests/client.rs`
3. `cargo test -p mlbt-api`

## Conventions

- Look for existing patterns before introducing new ones; shared color/display helpers live in `src/components/util.rs`.
- Match the style of surrounding code.
- Color and glyphs are **redundant** encoding — never make meaning depend on color alone. Text labels stay present; glyphs gate behind `nerd_fonts`; team colors gate behind `team_colors`; stat-tier coloring gates behind `theme` levels (`lean` / `classic` / `rainbow`).
- Branches off `main`; PRs squash-merged; include screenshots for visual changes.

## Working with this fork

- Upstream remote is `git@github.com:mlb-rs/mlbt.git`. Origin is `agiacalone/mlbtg`.
- When using `gh`, pass `--repo agiacalone/mlbtg` to avoid hitting upstream by mistake.
- Design notes for the fork's additions live in `docs/superpowers/specs/`.
