# Fork Divergence and Upstream v0.4.0 Merge — Design

**Status:** approved 2026-05-05
**Supersedes posture set in:** `2026-04-15-nerd-fonts-design.md`, `2026-04-16-contextual-colors-design.md` (their *features* survive; their "off-by-default" posture does not)

## Context

Upstream `mlb-rs/mlbt` shipped v0.4.0, which is a single feature commit (#141, "Refactor all styling and color handling") plus a release-version bump. PR #141 deletes `src/symbols.rs`, deletes `src/theme.rs`, introduces `src/ui/styling.rs` with flat `Color::Reset/Green/Red` helpers and a `DimStyle` trait, and removes the `&Symbols` parameter from every draw and widget API.

Our fork (`mlbtg`) had built a 3-tier `ThemeLevel` (Lean/Classic/Rainbow) on top of `Symbols`, plus a 4-tier Fangraphs-RGB stat-color system (`EXCELLENT/GOOD/BELOW_AVG/POOR/DIMMED`) with `Option<Color>` mid-range fallbacks across OBP/SLG/OPS/WHIP/ERA/AVG/win-pct, plus W/L coloring, multi-hit/HR highlights, and probable-pitcher ERA color. All of this was gated `off by default` to keep upstream-PR-compatibility.

That posture is dead. The original maintainer is not interested in glyphs (which is literally what the `g` in `mlbtg` stands for). We are a proper fork now and will take the program in our own direction. The architectural reset is: stop treating upstream as something we feed PRs back to, start treating it as a frame our work layers on top of.

## Goals

1. Merge upstream v0.4.0 (commits `b12b159` + `2001996`) into `main`.
2. Reorient the fork's architecture around two principles:
   - **Frame and layer.** Upstream owns chrome; we own signal. The boundary is enforced by file ownership.
   - **Glyphs and color are the product.** No toggle to disable them. A Nerd Font terminal is a hard requirement.
3. Preserve all behavior of our existing color and glyph work; reroute its imports to fit the new architecture.
4. Document the hot-metal-compositor newspaper-typography aesthetic as the project's design north star, so future visual passes have a coherent direction.

## Non-Goals

- Implementing newspaper-typography UI passes (en-dash scores, manicules, leader dots, drop caps, ornamental rules, dagger footnotes). These are tracked as separate follow-up specs, one per surface.
- Adding new statistical color dimensions beyond what currently ships.
- Re-evaluating the binary name, config path, or workspace layout.
- Backwards compatibility with old configs that referenced removed keys beyond silent-ignore.

## Architecture

### Frame: `src/ui/styling.rs` — upstream-owned

This file is taken verbatim from upstream PR #141. We do not edit it. It contains:
- `TEXT_COLOR`, `BORDER_COLOR`, `UNDERLINE_COLOR` const items (all `Color::Reset`).
- `border_style()`, `header_style()`, `text_style()`, `dim_style()`, `selected_style()` — chrome helpers returning `Style`.
- `DimStyle` trait — `is_zero()` + `dim_or_default() -> Style` for `u8`, `u16`, `str`.
- Flat `era_color`, `avg_color`, `win_pct_color` returning `Color` directly. (We *will* shadow these from `palette.rs` at our callsites, but the upstream-flavored versions stay in this file as the frame.)
- `era_style`, `avg_style` — `Style` siblings of the flat color fns.
- `convert_color` — RGBA-string-to-`Color::Rgb` parser used for team colors.

When upstream ships further styling work, we cherry-pick it onto this file as a fast-forward.

### Layer: `src/ui/palette.rs` — ours, new

A new sibling module that is the home of the mlbtg visual identity. It absorbs the entire public API of the old `src/theme.rs` minus the tier-branching machinery, plus the stat-tier color functions that today live in `src/components/util.rs`.

It exports:

```rust
// Fangraphs-derived stat-tier palette (formerly Theme::*).
pub const EXCELLENT: Color = Color::Rgb(69, 133, 207);
pub const GOOD: Color      = Color::Rgb(131, 178, 224);
pub const BELOW_AVG: Color = Color::Rgb(214, 153, 33);
pub const POOR: Color      = Color::Rgb(204, 36, 29);
pub const DIMMED: Color    = Color::Rgb(146, 131, 116);

// Chrome accents.
pub const ACCENT_BG: Color = Color::Rgb(69, 133, 136);
pub const ACCENT_FG: Color = Color::Rgb(235, 219, 178);
pub const BORDER: Color    = Color::Rgb(80, 73, 69);
pub const POSITIVE: Color  = Color::Rgb(152, 151, 26);
pub const TITLE_BG: Color  = Color::Rgb(40, 60, 80);
pub const TITLE_FG: Color  = Color::Rgb(235, 219, 178);

// Semantic backgrounds.
pub const ROW_HIGHLIGHT: Color = Color::Rgb(55, 55, 60);
pub const FAVORITE_BG: Color   = Color::Rgb(60, 48, 20);
pub const LIVE_GAME_BG: Color  = Color::Rgb(20, 45, 30);

// Stat-tier color functions (4-tier with mid-range fallback to None).
pub fn era_color(era: &str) -> Option<Color>;
pub fn avg_color(avg: &str) -> Option<Color>;
pub fn win_pct_color(pct: &str) -> Option<Color>;
pub fn obp_color(obp: &str) -> Option<Color>;
pub fn slg_color(slg: &str) -> Option<Color>;
pub fn ops_color(ops: &str) -> Option<Color>;
pub fn whip_color(whip: &str) -> Option<Color>;

// Weather scales.
pub fn temp_color(temp: u8) -> Color;
pub fn wind_color(mph: u8) -> Color;

// Chrome convenience accessors (formerly Theme methods, now tier-free —
// always return the rich-branch values).
pub fn border_color() -> Color;        // returns BORDER
pub fn dimmed_color() -> Color;        // returns DIMMED
pub fn selection_style() -> Style;     // ACCENT_BG/ACCENT_FG
pub fn title_style() -> Style;         // TITLE_FG/TITLE_BG
pub fn stat_style(fg_color: Color) -> Style;  // fg only — no rainbow backgrounds anymore
```

Behavior: identical to the rich-tier branch of today's `Theme` methods. `stat_style` collapses from a 3-branch tier match to a single line returning `Style::default().fg(fg_color)` — the rainbow background path is removed with the rainbow tier.

The seven existing `stat_color_tests` (28 unit tests covering OBP/SLG/OPS/WHIP/AVG/ERA/win-pct including zero cases) move into this file unchanged.

Callsites that previously imported stat colors from `crate::components::util` import them from `crate::ui::palette`. Callsites that previously called `self.symbols.theme().selection_style()` switch to `crate::ui::palette::selection_style()`. Callsites that referenced `Theme::EXCELLENT`/`Theme::DIMMED`/etc. switch to `crate::ui::palette::EXCELLENT`/etc.

Where stat-color names overlap with upstream's flat versions in `styling.rs` (`era_color`, `avg_color`, `win_pct_color`), the import path determines which is used; our callsites prefer ours. Upstream's flat versions stay reachable via `crate::ui::styling::*` for any callsite that genuinely wants them (currently none — but the option exists for future cherry-picks).

### Layer: `src/symbols.rs` — ours, slimmed

The `Symbols` struct is deleted. The glyph table and team-color table become free items in `src/symbols.rs`:

```rust
pub fn glyph_for(state: GameState) -> &'static str;
pub fn team_color(team_id: u32) -> Color;
// + any other lookups that today live on Symbols methods
```

No constructor, no `nerd_fonts: bool` field, no `theme: Theme` field, no `team_colors: bool` field. Glyphs render unconditionally; team colors apply unconditionally; there is no Lean/Classic/Rainbow tier to branch on.

The `&Symbols` parameter is removed from every `draw_*` function, every `Widget::render`/`StatefulWidget::render`, and every `to_cells`/`to_row_cells` method — matching upstream's signature shape. Callsites switch from `self.symbols.glyph_for(...)` / `symbols.team_color(...)` to `crate::symbols::glyph_for(...)` / `crate::symbols::team_color(...)`.

### Deletions

- `src/theme.rs` — entire file. The `Theme` struct, `ThemeLevel` enum, `EXCELLENT_BG`/`GOOD_BG`/`BELOW_AVG_BG`/`POOR_BG`/`ACCENT_BG`/`ACCENT_FG`/`BORDER`/`POSITIVE` background and chrome constants, and any `use_palette`/`use_backgrounds` tier-query methods.
- `mod theme;` in `src/lib.rs` or `src/main.rs` — wherever it's declared.
- All `_BG` background applications in render code (rainbow tier; gone with the tier).
- `theme_level`, `nerd_fonts`, `team_colors` fields from `Settings` / `config.rs` and any associated CLI flags or settings-editor UI rows.

### Configuration

`~/.config/mlbt/mlbt.toml` schema shrinks by three keys:

- Removed: `theme_level`, `nerd_fonts`, `team_colors`.
- Unchanged: `timezone`, `favorite_team`, refresh-interval keys, and any other non-color/non-glyph keys.

Old configs that contain the removed keys must not error. Mechanism: serde's default behavior ignores unknown fields. Verify the `Settings` struct does **not** carry `#[serde(deny_unknown_fields)]`; if it does, remove that attribute. No warning, no migration prompt — silent ignore.

### Settings editor

If the in-app settings editor (`src/state/settings_editor.rs`) currently exposes rows for the three removed keys, those rows are deleted. Adjust row indices and tests accordingly.

## Behavior preservation matrix

The merge must not change the rendered output of these surfaces (other than the always-on flip):

- **Stats table (`src/ui/stats.rs`):** OBP/SLG/OPS/WHIP column tier coloring; AVG and ERA column tier coloring.
- **Player profile season stats (`src/components/stats/player_profile.rs`):** OBP/SLG/OPS/WHIP/AVG/ERA tier coloring.
- **Player profile splits:** OBP/SLG/WHIP tier coloring.
- **Player profile game log:** Multi-hit (`H ≥ 2`) and home-run (`HR ≥ 1`) highlights; W/L color in W/L column; opponent team color in `Opp` column (only the team abbrev, not the `@`/`vs` prefix); leading-space pad on `@` to align with `vs` rows.
- **Team page schedule:** W/L color on score cell driven by `is_win: Option<bool>` field on `TeamGame`.
- **Probable pitchers panel (`src/ui/probable_pitchers.rs`):** ERA tier color in the ERA cell. The `to_row_cells` method returns `Vec<Cell>` rather than `Vec<String>`.
- **Standings (`src/ui/standings.rs`):** win-percentage tier color.
- **Game state glyphs:** Live ⚾, final ⚑, scheduled, in-progress, delayed, postponed glyphs render unconditionally.
- **Team color tinting:** Team abbreviations and team-color cells render unconditionally.

A focused manual smoke test pass (live game during prime time, scheduled game, finished game, stats table sort across all colored columns, player profile, standings) is the acceptance gate; the existing test suite is the regression gate.

## Documentation changes

### `CLAUDE.md`

Replace the `## Project` section's "All additions are off by default — without config changes, behavior matches upstream." line with:

> `mlbtg` is Anthony's fork of [mlb-rs/mlbt](https://github.com/mlb-rs/mlbt). Upstream is the **frame**; our work is the **layer**. We do not feed PRs back to upstream and we do not gate features behind opt-in flags. Glyphs and color are the product — a Nerd Font terminal is required.

Add a new `## Design north star` section:

> The aesthetic target is hot-metal-compositor newspaper typography. Color is **signal**, not decoration: the Fangraphs blue→amber→red scale carries stat tier; W/L is green/red; live state has its own glyph. Glyphs are **marks**: ⚾ for live, ⚑ for final, manicules for current at-bat, dagger for ejected. Density over chrome — the stathead reads at a glance.

Add a new `## Frame and layer` section:

> - **Frame** — `src/ui/styling.rs` is upstream-owned. Do not edit. Cherry-pick upstream styling work onto it as fast-forwards.
> - **Layer** — `src/ui/palette.rs` and `src/symbols.rs` are ours. Stat tier colors and glyph lookups live here. Override or shadow frame helpers at the *callsite import line*; do not push our behavior into the frame file.

### `README.md`

Add near the top, under the project description:

> **Requirements:** a Nerd Font terminal (e.g. Iosevka Nerd Font, JetBrainsMono Nerd Font). Color and glyphs are unconditional.

### `CHANGELOG.md`

One entry under v1.1.0 (or the next available version):

```
## [1.1.0] - 2026-05-05

### Changed
- Merged upstream v0.4.0 (mlb-rs/mlbt #141 styling refactor + release).
- Adopted upstream's `src/ui/styling.rs` as the chrome frame.
- Moved Fangraphs stat colors into `src/ui/palette.rs` (sibling layer).
- Removed `theme_level`, `nerd_fonts`, `team_colors` config keys.
- Glyphs and rich color are now unconditional. A Nerd Font terminal is required.
```

## Testing

- All `stat_color_tests` (28 unit tests covering OBP/SLG/OPS/WHIP/AVG/ERA/win-pct tiers including zero handling) move from `src/components/util.rs` into `src/ui/palette.rs` unchanged. No coverage loss.
- `test_color_conversion` stays in `src/ui/styling.rs` alongside `convert_color` (both arrive verbatim from upstream PR #141).
- `test_last_name` stays in `src/components/util.rs` (last-name extraction is unrelated to color).
- All theme-tier tests (`use_palette`, `use_backgrounds`, anything in or referencing `src/theme.rs`) are deleted.
- CI gates unchanged: `cargo fmt --all -- --check`, `cargo clippy --all-features --all-targets --workspace --locked -- --deny warnings`, `cargo test --all-features --all-targets --workspace --locked`.
- Acceptance smoke test (manual): launch against a live MLB game, switch tabs (Scoreboard / Gameday / Stats / Standings), open a player profile, sort stats by OBP/SLG/OPS/WHIP and confirm tier colors render. Confirm Nerd Font glyphs render. Confirm team color tints render.

## Risks and mitigations

- **Risk:** silent visual regression in a callsite where the upstream-flavored `era_color` (returns `Color::Reset` for mid-range) gets imported instead of ours (returns `Option<Color>` → text color for mid-range). The two render the same on a default-themed terminal but the type signatures differ.
  - **Mitigation:** at the end of the merge, grep for `use crate::ui::styling::{*era_color*, *avg_color*, *win_pct_color*}` and confirm none of our callsites import the flat versions. Our callsites all use `Option<Color>`-returning fns.

- **Risk:** an old config in the wild contains `theme_level = "rainbow"` and breaks startup.
  - **Mitigation:** verify silent-ignore at runtime by writing a test config with the removed keys and asserting `Settings::load` succeeds.

- **Risk:** the `Symbols` removal touches more callsites than the diff suggests because of method-call ergonomics (`self.symbols.glyph_for(...)` patterns spread through render code).
  - **Mitigation:** the implementation plan resolves callsites file-by-file with `cargo build` between files; callsites that don't compile are flagged before the merge commit lands.

## Out of scope (follow-up specs)

Each of these gets its own design + plan after this merge ships:

- En-dash scores in linescore (`5–3` instead of `5-3`).
- Leader-dot rhythm in stat tables (`AVG · OBP · SLG · OPS`).
- Manicule glyph for current at-bat in gameday plays panel.
- Dagger footnote for ejected players in box score.
- Drop-cap section openers in player profile bio.
- Ornamental rule between innings in linescore.
- Small-caps treatment for column headers across all tables.
