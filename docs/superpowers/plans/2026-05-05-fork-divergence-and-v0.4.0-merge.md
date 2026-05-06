# Fork Divergence and Upstream v0.4.0 Merge — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Spec:** `docs/superpowers/specs/2026-05-05-fork-divergence-and-v0.4.0-merge-design.md`

**Goal:** Merge upstream `mlb-rs/mlbt` v0.4.0 (commits `b12b159` + `2001996`) into `main` while reorienting the fork's architecture so upstream's `src/ui/styling.rs` is the chrome frame and `src/ui/palette.rs` + `src/symbols.rs` are the mlbtg layer. Drop the `Theme`/`ThemeLevel` tier system, drop `Symbols` as a struct, drop the `theme_level`/`nerd_fonts`/`team_colors` config keys. Glyphs and rich color become unconditional.

**Architecture:** The work happens on a single merge branch. Phase 0 sets up the branch and begins the merge. Phase 1 resolves non-source conflicts. Phase 2 lands the new layer module (`palette.rs`) and rewrites `symbols.rs` to free functions. Phase 3 deletes `theme.rs` and shrinks the config. Phases 4–5 migrate callsites file-by-file in `components/` then `ui/`. Phase 6 verifies. Phase 7 finalizes docs and the merge commit.

**Tech Stack:** Rust 2024 edition, ratatui (`tui` crate alias), serde, GPG-signed commits.

**Verification gates** (run from the repo root):
- `cargo build --workspace --locked` — must succeed at every checkpoint
- `cargo fmt --all -- --check` — must pass before commit
- `cargo clippy --workspace --all-targets --all-features --locked -- --deny warnings` — must pass before commit
- `cargo test --workspace --all-targets --all-features --locked` — must pass before merge commit lands

**Commit posture:** Anthony's git config requires GPG-signed commits (`-S` is implicit via `commit.gpgsign = true`). Intermediate commits on the merge branch are signed individually if pinentry is reachable; otherwise the executor coalesces work and signs a single merge commit at the end. Tasks marked `[checkpoint]` are natural commit boundaries; tasks marked `[continuation]` should not commit independently and instead build atop the previous checkpoint.

**Fallback policy:** If any task fails its verification gate and the cause is not a typo or a missed import, **stop and report**. Do not attempt aggressive resolutions (deleting tests, suppressing warnings, `git checkout --ours`/`--theirs` outside Phase 1). Anthony triages.

---

## File Structure

### Created
- `src/ui/palette.rs` — mlbtg's color and chrome identity. Stat tier palette, weather scales, semantic backgrounds, chrome accents, chrome accessor functions, all stat-tier color fns. Single responsibility: "what colors and styled chrome look like in mlbtg."

### Deleted
- `src/theme.rs` — entire file. Tier toggle plus all the constants and methods it owned.

### Rewritten
- `src/symbols.rs` — `Symbols` struct deleted. Glyph methods become free `pub fn`s that always return the Nerd Fonts variant. `weather_icon`/`format_weather`/`format_wind` likewise.
- `src/components/util.rs` — color fns and `DimColor` trait moved out (to palette / replaced by upstream's `DimStyle`). `OptionDisplayExt`, `OptionMapDisplayExt`, `last_name` and their tests stay.

### Modified (callsite migration only — behavior preserved)
- `src/main.rs` (or wherever `mod theme;` lives) — drop `mod theme;`
- `src/ui/mod.rs` — add `pub mod palette;`, accept upstream's `pub mod styling;`
- `src/config.rs` — drop `nerd_fonts`/`team_colors`/`theme` from `ConfigFile`
- `src/state/app_settings.rs` — drop the three corresponding fields from `AppSettings`
- `src/draw.rs` — accept upstream's signature changes; rewire chrome to `palette::*`
- `src/components/standings.rs`, `src/components/linescore.rs`, `src/components/probable_pitchers.rs`, `src/components/boxscore.rs`, `src/components/team_page.rs`, `src/components/stats/player_profile.rs`, `src/components/stats/table.rs`, `src/components/game/matchup.rs`, `src/components/game/pitch_event.rs`, `src/components/game/pitches.rs`, `src/components/game/strikezone.rs`
- `src/ui/standings.rs`, `src/ui/stats.rs`, `src/ui/schedule.rs`, `src/ui/player_profile.rs`, `src/ui/team_page.rs`, `src/ui/decision_pitchers.rs`, `src/ui/linescore.rs`, `src/ui/boxscore.rs`, `src/ui/scroll.rs`
- `src/ui/gameday/at_bat.rs`, `src/ui/gameday/gameday_widget.rs`, `src/ui/gameday/matchup.rs`, `src/ui/gameday/plays.rs`, `src/ui/gameday/win_probability.rs`

### Documentation
- `CLAUDE.md`, `README.md`, `CHANGELOG.md` — per spec §4
- `Cargo.toml`, `Cargo.lock` — version bump + upstream dep sync

---

## Phase 0: Preflight

### Task 0.1: Establish a clean merge branch [checkpoint]

**Files:** none — git operations only.

- [ ] **Step 1:** Confirm the working tree is clean.

  Run: `git status`
  Expected: `nothing to commit, working tree clean` and current branch is `main`.

- [ ] **Step 2:** Confirm `upstream/main` is at `2001996`.

  Run: `git fetch upstream && git log -1 --oneline upstream/main`
  Expected: `2001996 Release version 0.4.0 (#142)`.

- [ ] **Step 3:** Create the merge branch from `main`.

  Run: `git checkout -b merge/upstream-v0.4.0`
  Expected: `Switched to a new branch 'merge/upstream-v0.4.0'`.

### Task 0.2: Verify config struct does not deny unknown fields [continuation]

**Files:**
- Read: `src/config.rs`

- [ ] **Step 1:** Verify `ConfigFile` has no `#[serde(deny_unknown_fields)]`.

  Run: `grep -n "deny_unknown_fields" src/config.rs`
  Expected: no output.

  If output is non-empty, **stop and report** — the spec assumes silent-ignore via serde's default behavior. Removing that attribute requires a separate decision.

---

## Phase 1: Start the merge, resolve non-source conflicts

### Task 1.1: Begin the merge [continuation]

**Files:** none — git operation only.

- [ ] **Step 1:** Begin the merge without committing.

  Run: `git merge upstream/main --no-commit --no-ff`
  Expected: 17 `CONFLICT (content)` lines and `Automatic merge failed; fix conflicts and then commit the result.`

- [ ] **Step 2:** Confirm the conflicted file list matches the spec's expectation.

  Run: `git diff --name-only --diff-filter=U | sort`
  Expected (17 files):
  ```
  CHANGELOG.md
  Cargo.lock
  Cargo.toml
  src/components/linescore.rs
  src/components/standings.rs
  src/components/stats/player_profile.rs
  src/components/util.rs
  src/draw.rs
  src/ui/decision_pitchers.rs
  src/ui/gameday/at_bat.rs
  src/ui/gameday/matchup.rs
  src/ui/gameday/plays.rs
  src/ui/linescore.rs
  src/ui/player_profile.rs
  src/ui/schedule.rs
  src/ui/standings.rs
  src/ui/stats.rs
  src/ui/team_page.rs
  ```

  If the list differs, **stop and report**.

### Task 1.2: Resolve `Cargo.toml` [continuation]

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1:** Open `Cargo.toml`, locate the `<<<<<<<`/`=======`/`>>>>>>>` markers.

- [ ] **Step 2:** Resolve by keeping our `[package]` block (`name = "mlbtg"`, version bumped to `1.1.0`, our authors list, our repository URL) and accepting upstream's dependency-version updates everywhere else.

  Concrete rules:
  - `name`: keep `"mlbtg"` (ours).
  - `version`: set to `"1.1.0"` (new — neither side's value).
  - `authors`: keep ours (the two-author list including Anthony).
  - `repository`: keep ours (`agiacalone/mlbtg`).
  - All `[dependencies]`, `[workspace.dependencies]`, `[dev-dependencies]`: take upstream's versions wherever they bumped.

- [ ] **Step 3:** Remove all conflict markers.

- [ ] **Step 4:** Verify the file parses.

  Run: `cargo metadata --no-deps --format-version 1 > /dev/null`
  Expected: no error.

### Task 1.3: Resolve `Cargo.lock` [continuation]

**Files:**
- Modify: `Cargo.lock`

- [ ] **Step 1:** Take upstream's `Cargo.lock` wholesale (their version pins win).

  Run: `git checkout --theirs Cargo.lock`

- [ ] **Step 2:** Regenerate to reflect our `Cargo.toml` version bump.

  Run: `cargo update --workspace --offline 2>/dev/null || cargo generate-lockfile`
  Expected: `Cargo.lock` updates the `mlbtg` package version to `1.1.0`. No other packages should churn.

- [ ] **Step 3:** Stage the lock file.

  Run: `git add Cargo.lock`

### Task 1.4: Resolve `CHANGELOG.md` [continuation]

**Files:**
- Modify: `CHANGELOG.md`

- [ ] **Step 1:** Take upstream's CHANGELOG content wholesale, then prepend a new `1.1.0` entry above their topmost entry.

  Run: `git checkout --theirs CHANGELOG.md`

- [ ] **Step 2:** Insert at the top of `CHANGELOG.md`, immediately under any leading `# Changelog` header (preserve the header if present):

  ```markdown
  ## [1.1.0] - 2026-05-05

  ### Changed
  - Merged upstream v0.4.0 (mlb-rs/mlbt #141 styling refactor + release).
  - Adopted upstream's `src/ui/styling.rs` as the chrome frame.
  - Moved Fangraphs stat colors and chrome palette into `src/ui/palette.rs` (sibling layer).
  - Removed `theme_level`, `nerd_fonts`, `team_colors` config keys.
  - Glyphs and rich color are now unconditional. A Nerd Font terminal is required.
  ```

- [ ] **Step 3:** Stage the file.

  Run: `git add CHANGELOG.md`

### Task 1.5: Commit the non-source resolution [checkpoint]

**Files:** none — git operation only.

- [ ] **Step 1:** Confirm only the three resolved files are staged for commit and the merge is still in progress.

  Run: `git status`
  Expected: `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` listed under "Changes to be committed"; the rest of the conflicted files still under "Unmerged paths".

- [ ] **Step 2:** Do **not** commit yet — the merge is one commit and lands at the end (Task 7.5). Just confirm staged state.

---

## Phase 2: Land the layer module

### Task 2.1: Take upstream's `src/ui/styling.rs` verbatim [continuation]

**Files:**
- Create (from upstream): `src/ui/styling.rs`

- [ ] **Step 1:** Pull upstream's version of the new file.

  Run: `git show upstream/main:src/ui/styling.rs > src/ui/styling.rs && git add src/ui/styling.rs`

- [ ] **Step 2:** Confirm the file is staged and reads as expected (157 lines, contains `pub const TEXT_COLOR`, `pub fn border_style`, `pub trait DimStyle`, `pub fn era_color`, `pub fn convert_color`).

  Run: `wc -l src/ui/styling.rs && grep -c "^pub" src/ui/styling.rs`
  Expected: `157 src/ui/styling.rs` and at least 10 `pub` items.

### Task 2.2: Add `pub mod styling` and `pub mod palette` to `src/ui/mod.rs` [continuation]

**Files:**
- Modify: `src/ui/mod.rs`

- [ ] **Step 1:** Resolve the conflict in `src/ui/mod.rs` by accepting upstream's `pub mod styling;` line and appending our own `pub mod palette;` line.

  After resolution the file ends with:
  ```rust
  pub(crate) mod schedule;
  pub(crate) mod scroll;
  pub(crate) mod standings;
  pub(crate) mod stats;
  pub mod styling;
  pub(crate) mod palette;
  pub(crate) mod team_page;
  ```

  (Order: maintain alphabetical; insert `palette` between `mod` and `team_page`. The file may differ slightly — adjust to alphabetical.)

  Run: `git diff --no-color src/ui/mod.rs` afterward and confirm no `<<<` markers remain.

### Task 2.3: Create `src/ui/palette.rs` [checkpoint]

**Files:**
- Create: `src/ui/palette.rs`

- [ ] **Step 1:** Write the file with the full content below.

  ```rust
  //! mlbtg's color and chrome identity.
  //!
  //! This module is the *layer* that sits atop upstream's `src/ui/styling.rs` *frame*.
  //! It owns the Fangraphs stat-tier palette, weather signal scales, semantic
  //! backgrounds, chrome accent constants, and chrome accessor functions.
  //!
  //! Stat-tier color functions (`era_color`, `avg_color`, `win_pct_color`,
  //! `obp_color`, `slg_color`, `ops_color`, `whip_color`) shadow upstream's
  //! flat versions in `styling.rs`. Our callsites prefer ours.

  use tui::style::{Color, Style};

  // --- Fangraphs-derived stat-tier palette ---

  pub const EXCELLENT: Color = Color::Rgb(69, 133, 207);
  pub const GOOD: Color = Color::Rgb(131, 178, 224);
  pub const BELOW_AVG: Color = Color::Rgb(214, 153, 33);
  pub const POOR: Color = Color::Rgb(204, 36, 29);
  pub const DIMMED: Color = Color::Rgb(146, 131, 116);

  // --- Chrome accents ---

  pub const ACCENT_BG: Color = Color::Rgb(69, 133, 136);
  pub const ACCENT_FG: Color = Color::Rgb(235, 219, 178);
  pub const BORDER: Color = Color::Rgb(80, 73, 69);
  pub const POSITIVE: Color = Color::Rgb(152, 151, 26);
  pub const TITLE_BG: Color = Color::Rgb(40, 60, 80);
  pub const TITLE_FG: Color = Color::Rgb(235, 219, 178);

  // --- Semantic backgrounds ---

  pub const ROW_HIGHLIGHT: Color = Color::Rgb(55, 55, 60);
  pub const FAVORITE_BG: Color = Color::Rgb(60, 48, 20);
  pub const LIVE_GAME_BG: Color = Color::Rgb(20, 45, 30);

  // --- Chrome convenience accessors ---

  pub fn border_color() -> Color {
      BORDER
  }

  pub fn dimmed_color() -> Color {
      DIMMED
  }

  pub fn selection_style() -> Style {
      Style::default().bg(ACCENT_BG).fg(ACCENT_FG)
  }

  pub fn title_style() -> Style {
      Style::default().fg(TITLE_FG).bg(TITLE_BG)
  }

  /// Returns a style for a stat cell with the given foreground color.
  /// Always foreground-only — the rainbow-tier background variants were
  /// removed when the tier toggle was removed.
  pub fn stat_style(fg_color: Color) -> Style {
      Style::default().fg(fg_color)
  }

  // --- Stat-tier color functions ---

  /// Color for an ERA stat string. Returns `None` for the average range
  /// (3.00–4.99) so callsites fall back to default text color.
  pub fn era_color(era: &str) -> Option<Color> {
      era.parse::<f64>().ok().and_then(|v| {
          if v <= 2.50 {
              Some(EXCELLENT)
          } else if v <= 3.00 {
              Some(GOOD)
          } else if v >= 5.00 {
              Some(POOR)
          } else if v >= 4.00 {
              Some(BELOW_AVG)
          } else {
              None
          }
      })
  }

  /// Color for a batting-average stat string. Returns `None` for mid-range
  /// (.100–.299) so callsites fall back to default text color.
  pub fn avg_color(avg: &str) -> Option<Color> {
      avg.parse::<f64>().ok().and_then(|v| {
          if v == 0.0 {
              Some(DIMMED)
          } else if v >= 0.300 {
              Some(EXCELLENT)
          } else if v >= 0.275 {
              Some(GOOD)
          } else if v < 0.100 {
              Some(POOR)
          } else if v < 0.200 {
              Some(BELOW_AVG)
          } else {
              None
          }
      })
  }

  /// Color for a winning-percentage stat string.
  pub fn win_pct_color(pct: &str) -> Option<Color> {
      pct.parse::<f64>().ok().map(|v| {
          if v == 0.0 {
              DIMMED
          } else if v >= 0.600 {
              EXCELLENT
          } else if v >= 0.500 {
              GOOD
          } else if v >= 0.400 {
              BELOW_AVG
          } else {
              POOR
          }
      })
  }

  /// Color for an OBP stat string. Returns `None` for the average range (.290–.349).
  pub fn obp_color(obp: &str) -> Option<Color> {
      obp.parse::<f64>().ok().and_then(|v| {
          if v == 0.0 {
              Some(DIMMED)
          } else if v >= 0.380 {
              Some(EXCELLENT)
          } else if v >= 0.350 {
              Some(GOOD)
          } else if v < 0.250 {
              Some(POOR)
          } else if v < 0.290 {
              Some(BELOW_AVG)
          } else {
              None
          }
      })
  }

  /// Color for a SLG stat string. Returns `None` for the average range (.350–.449).
  pub fn slg_color(slg: &str) -> Option<Color> {
      slg.parse::<f64>().ok().and_then(|v| {
          if v == 0.0 {
              Some(DIMMED)
          } else if v >= 0.500 {
              Some(EXCELLENT)
          } else if v >= 0.450 {
              Some(GOOD)
          } else if v < 0.300 {
              Some(POOR)
          } else if v < 0.350 {
              Some(BELOW_AVG)
          } else {
              None
          }
      })
  }

  /// Color for an OPS stat string. Returns `None` for the average range (.680–.799).
  pub fn ops_color(ops: &str) -> Option<Color> {
      ops.parse::<f64>().ok().and_then(|v| {
          if v == 0.0 {
              Some(DIMMED)
          } else if v >= 0.900 {
              Some(EXCELLENT)
          } else if v >= 0.800 {
              Some(GOOD)
          } else if v < 0.600 {
              Some(POOR)
          } else if v < 0.680 {
              Some(BELOW_AVG)
          } else {
              None
          }
      })
  }

  /// Color for a WHIP stat string. Returns `None` for the average range (1.11–1.39).
  /// Lower WHIP is better — color scale is inverted relative to batting stats.
  /// 0.00 WHIP is treated as elite performance (no hits or walks allowed).
  pub fn whip_color(whip: &str) -> Option<Color> {
      whip.parse::<f64>().ok().and_then(|v| {
          if v <= 0.90 {
              Some(EXCELLENT)
          } else if v <= 1.10 {
              Some(GOOD)
          } else if v >= 1.60 {
              Some(POOR)
          } else if v >= 1.40 {
              Some(BELOW_AVG)
          } else {
              None
          }
      })
  }

  // --- Weather signal scales ---

  /// Color for temperature on a cold-to-hot scale.
  pub fn temp_color(temp: u8) -> Color {
      match temp {
          0..=45 => Color::Rgb(100, 150, 220),  // cold — blue
          46..=60 => Color::Rgb(80, 170, 170),  // cool — teal
          61..=80 => Color::Rgb(146, 131, 116), // mild — dimmed/neutral
          81..=95 => Color::Rgb(214, 153, 33),  // warm — orange
          _ => Color::Rgb(204, 36, 29),         // hot — red
      }
  }

  /// Color for wind speed. Calm is dimmed, strong wind gets more visible.
  pub fn wind_color(mph: u8) -> Color {
      match mph {
          0..=5 => Color::Rgb(146, 131, 116),  // calm — dimmed
          6..=12 => Color::Rgb(131, 178, 224), // light — soft blue
          13..=20 => Color::Rgb(69, 133, 207), // moderate — blue
          _ => Color::Rgb(160, 90, 200),       // strong — purple
      }
  }

  // -------------------------------------------------------------------------
  // Tests
  // -------------------------------------------------------------------------

  #[cfg(test)]
  mod stat_color_tests {
      use super::*;

      #[test]
      fn obp_excellent() {
          assert_eq!(obp_color(".400"), Some(EXCELLENT));
      }
      #[test]
      fn obp_good() {
          assert_eq!(obp_color(".355"), Some(GOOD));
      }
      #[test]
      fn obp_average() {
          assert_eq!(obp_color(".310"), None);
      }
      #[test]
      fn obp_below() {
          assert_eq!(obp_color(".270"), Some(BELOW_AVG));
      }
      #[test]
      fn obp_poor() {
          assert_eq!(obp_color(".240"), Some(POOR));
      }
      #[test]
      fn obp_zero() {
          assert_eq!(obp_color(".000"), Some(DIMMED));
      }

      #[test]
      fn slg_excellent() {
          assert_eq!(slg_color(".520"), Some(EXCELLENT));
      }
      #[test]
      fn slg_good() {
          assert_eq!(slg_color(".460"), Some(GOOD));
      }
      #[test]
      fn slg_average() {
          assert_eq!(slg_color(".400"), None);
      }
      #[test]
      fn slg_below() {
          assert_eq!(slg_color(".320"), Some(BELOW_AVG));
      }
      #[test]
      fn slg_poor() {
          assert_eq!(slg_color(".280"), Some(POOR));
      }
      #[test]
      fn slg_zero() {
          assert_eq!(slg_color(".000"), Some(DIMMED));
      }

      #[test]
      fn ops_excellent() {
          assert_eq!(ops_color(".950"), Some(EXCELLENT));
      }
      #[test]
      fn ops_good() {
          assert_eq!(ops_color(".820"), Some(GOOD));
      }
      #[test]
      fn ops_average() {
          assert_eq!(ops_color(".730"), None);
      }
      #[test]
      fn ops_below() {
          assert_eq!(ops_color(".640"), Some(BELOW_AVG));
      }
      #[test]
      fn ops_poor() {
          assert_eq!(ops_color(".550"), Some(POOR));
      }
      #[test]
      fn ops_zero() {
          assert_eq!(ops_color(".000"), Some(DIMMED));
      }

      #[test]
      fn whip_excellent() {
          assert_eq!(whip_color("0.85"), Some(EXCELLENT));
      }
      #[test]
      fn whip_good() {
          assert_eq!(whip_color("1.05"), Some(GOOD));
      }
      #[test]
      fn whip_average() {
          assert_eq!(whip_color("1.25"), None);
      }
      #[test]
      fn whip_below() {
          assert_eq!(whip_color("1.45"), Some(BELOW_AVG));
      }
      #[test]
      fn whip_poor() {
          assert_eq!(whip_color("1.70"), Some(POOR));
      }
      #[test]
      fn whip_zero() {
          assert_eq!(whip_color("0.00"), Some(EXCELLENT));
      }
  }
  ```

- [ ] **Step 2:** Verify the file compiles in isolation.

  Run: `cargo build --workspace --locked 2>&1 | tail -20`
  Expected: build will fail because callsites still reference `theme.rs` and the old `util.rs` color fns. **That's OK at this point** — we're using build to confirm `palette.rs` itself has no syntax errors. Look for errors with paths starting `src/ui/palette.rs`. There should be none.

- [ ] **Step 3:** Run only the palette tests.

  Run: `cargo test --workspace --locked --no-fail-fast palette:: 2>&1 | tail -10`
  Expected: 24 stat-color tests pass (all of `obp_*`, `slg_*`, `ops_*`, `whip_*` plus the implicit count from the macro expansion). If the project doesn't build at all (because callsites are still wired to deleted `theme.rs`), this command will fail compilation — that's the next phase's problem, not this one. Skip this step if compilation isn't yet green and resume it after Phase 5.

### Task 2.4: Rewrite `src/symbols.rs` to free functions [checkpoint]

**Files:**
- Modify: `src/symbols.rs`

- [ ] **Step 1:** Replace the entire contents with the version below. The struct, the constructor, the `theme()` accessor, the `nerd_fonts()`/`team_colors()` boolean accessors, and all the `if self.nerd_fonts { ... } else { ... }` branches go away. Every glyph function returns the Nerd Fonts variant unconditionally.

  ```rust
  //! Nerd Font glyphs and weather/wind formatting.
  //!
  //! Glyphs are unconditional — a Nerd Font terminal is a hard requirement
  //! for mlbtg.

  /// Tab title icons.
  pub fn tab_scoreboard() -> &'static str {
      "\u{F073} "
  }

  pub fn tab_gameday() -> &'static str {
      "\u{F008} "
  }

  pub fn tab_stats() -> &'static str {
      "\u{F080} "
  }

  pub fn tab_standings() -> &'static str {
      "\u{F091} "
  }

  /// Cursor shown next to the selected play in the at-bat plays list.
  pub fn selection_cursor() -> char {
      '\u{F0DA}'
  }

  /// Indicator shown for scoring plays.
  pub fn scoring_play() -> char {
      '\u{F43F}'
  }

  /// Filled base (runner on base). Standard Unicode diamond — no PUA equivalent in Nerd Fonts.
  pub fn base_occupied() -> char {
      '◆'
  }

  /// Empty base. Standard Unicode diamond — no PUA equivalent in Nerd Fonts.
  pub fn base_empty() -> char {
      '◇'
  }

  /// Scrollbar begin symbol (top of content).
  pub fn scroll_up() -> &'static str {
      "\u{F062}"
  }

  /// Scrollbar end symbol (bottom of content).
  pub fn scroll_down() -> &'static str {
      "\u{F063}"
  }

  /// Sort ascending column header indicator.
  pub fn sort_asc() -> &'static str {
      "↑"
  }

  /// Sort descending column header indicator.
  pub fn sort_desc() -> &'static str {
      "↓"
  }

  /// Prefix shown before the favorite team's game. Always 2 chars wide.
  pub fn favorite_marker() -> &'static str {
      "★ "
  }

  /// Weather icon for the given condition string from the MLB API.
  pub fn weather_icon(condition: &str) -> &'static str {
      let lower = condition.to_lowercase();
      if lower.contains("sun") || lower.contains("clear") {
          "\u{E302}"
      } else if lower.contains("partly") || lower.contains("few clouds") {
          "\u{E37B}"
      } else if lower.contains("cloud") || lower.contains("overcast") {
          "\u{E312}"
      } else if lower.contains("rain") || lower.contains("drizzle") || lower.contains("shower") {
          "\u{E318}"
      } else if lower.contains("snow") {
          "\u{E31A}"
      } else if lower.contains("thunder") || lower.contains("storm") {
          "\u{E31D}"
      } else if lower.contains("fog") || lower.contains("mist") || lower.contains("haze") {
          "\u{E313}"
      } else if lower.contains("wind") {
          "\u{E34B}"
      } else if lower.contains("dome") || lower.contains("roof") {
          "\u{F015}"
      } else {
          "\u{E302}"
      }
  }

  /// Format a weather string for display, e.g. "☀ 72°F".
  pub fn format_weather(condition: &str, temp: &str) -> String {
      let icon = weather_icon(condition);
      format!("{icon} {temp}°F")
  }

  /// Format wind string from the API into compact arrow notation.
  /// "11 mph, Out To RF" -> "11 mph ↗"
  pub fn format_wind(wind: &str) -> String {
      let (speed, direction) = match wind.split_once(", ") {
          Some((s, d)) => (s, Some(d)),
          None => (wind, None),
      };
      let Some(dir) = direction else {
          return speed.to_string();
      };
      let arrow = match dir {
          "Out To RF" => "↗",
          "Out To CF" => "↑",
          "Out To LF" => "↖",
          "In From RF" => "↙",
          "In From CF" => "↓",
          "In From LF" => "↘",
          "R To L" => "←",
          "L To R" => "→",
          "Calm" => "·",
          _ => dir,
      };
      format!("{speed} {arrow}")
  }

  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn glyphs_are_nerd_fonts_variants() {
          assert_eq!(tab_scoreboard(), "\u{F073} ");
          assert_eq!(tab_gameday(), "\u{F008} ");
          assert_eq!(tab_stats(), "\u{F080} ");
          assert_eq!(tab_standings(), "\u{F091} ");
          assert_eq!(selection_cursor(), '\u{F0DA}');
          assert_eq!(scoring_play(), '\u{F43F}');
          assert_eq!(base_occupied(), '◆');
          assert_eq!(base_empty(), '◇');
          assert_eq!(scroll_up(), "\u{F062}");
          assert_eq!(scroll_down(), "\u{F063}");
          assert_eq!(sort_asc(), "↑");
          assert_eq!(sort_desc(), "↓");
          assert_eq!(favorite_marker(), "★ ");
      }

      #[test]
      fn format_wind_with_arrow() {
          assert_eq!(format_wind("11 mph, Out To RF"), "11 mph ↗");
          assert_eq!(format_wind("7 mph, R To L"), "7 mph ←");
          assert_eq!(format_wind("Calm"), "Calm");
      }

      #[test]
      fn format_weather_with_icon() {
          assert!(format_weather("Sunny", "72").starts_with("\u{E302}"));
          assert!(format_weather("Sunny", "72").ends_with("72°F"));
      }
  }
  ```

- [ ] **Step 2:** Stage the file.

  Run: `git add src/symbols.rs`

---

## Phase 3: Delete `theme.rs` and shrink config

### Task 3.1: Delete `src/theme.rs` and remove its module declaration [checkpoint]

**Files:**
- Delete: `src/theme.rs`
- Modify: `src/main.rs` (or wherever `mod theme;` is declared)

- [ ] **Step 1:** Delete the file.

  Run: `git rm src/theme.rs`
  Expected: `rm 'src/theme.rs'`.

- [ ] **Step 2:** Locate the `mod theme;` declaration.

  Run: `grep -rn "^mod theme;\|^pub mod theme;" src/`
  Expected: one or two hits, typically in `src/main.rs` or `src/lib.rs`.

- [ ] **Step 3:** Delete that line. Build to confirm the deletion is clean (this build will still fail elsewhere — we're only checking that `mod theme` removal doesn't introduce new issues).

  Run: `cargo build --workspace --locked 2>&1 | grep "no module named .theme." || echo "ok: theme module reference clean"`
  Expected: `ok: theme module reference clean` or no output.

### Task 3.2: Drop config keys [continuation]

**Files:**
- Modify: `src/config.rs`
- Modify: `src/state/app_settings.rs`

- [ ] **Step 1:** In `src/config.rs`:
  - Remove the line `use crate::theme::ThemeLevel;`
  - Remove the `nerd_fonts: Option<bool>`, `team_colors: Option<bool>`, `theme: Option<ThemeLevel>` fields from `ConfigFile`.
  - Remove `nerd_fonts: None`, `team_colors: None`, `theme: None` from `Default for ConfigFile`.
  - In `From<ConfigFile> for AppSettings`, remove the three `nerd_fonts: file.nerd_fonts.unwrap_or(false)`, `team_colors: file.team_colors.unwrap_or(false)`, `theme: file.theme.unwrap_or_default()` lines.
  - In `From<&AppSettings> for ConfigFile`, remove the three corresponding `s.nerd_fonts`, `s.team_colors`, `s.theme` lines.

- [ ] **Step 2:** In `src/state/app_settings.rs`:
  - Remove the line `use crate::theme::ThemeLevel;`.
  - Remove the `pub nerd_fonts: bool`, `pub team_colors: bool`, `pub theme: ThemeLevel` fields from `AppSettings`.

- [ ] **Step 3:** Stage both files.

  Run: `git add src/config.rs src/state/app_settings.rs`

### Task 3.3: Remove settings-editor rows for the dropped keys [continuation]

**Files:**
- Modify (if present): `src/state/settings_editor.rs`

- [ ] **Step 1:** Check whether the in-app settings editor exposes rows for the three removed keys.

  Run: `grep -n "nerd_fonts\|team_colors\|theme" src/state/settings_editor.rs 2>/dev/null || echo "no settings_editor.rs"`
  Expected: either no file, or a list of hits.

- [ ] **Step 2:** If hits exist, remove all rows / row-handlers / row-index references that touch `nerd_fonts`, `team_colors`, or `theme`. Adjust any row-count constants and adjust any tests that reference row indices. If a test file depends on these rows, update the test alongside.

  If no `settings_editor.rs` exists, skip this task.

- [ ] **Step 3:** Stage if modified.

  Run: `git add src/state/settings_editor.rs 2>/dev/null || true`

### Task 3.4: Add a regression test for old-config silent-ignore [continuation]

**Files:**
- Modify: `src/config.rs` (test module)

- [ ] **Step 1:** Add this test inside the existing `#[cfg(test)] mod tests` block in `src/config.rs` (create the block if it doesn't exist):

  ```rust
  #[test]
  fn old_config_with_removed_keys_loads() {
      let toml = r#"
          favorite_team = "Giants"
          timezone = "US/Pacific"
          theme = "rainbow"
          nerd_fonts = true
          team_colors = true
      "#;
      let parsed: ConfigFile =
          toml::from_str(toml).expect("config with removed keys should parse");
      // Surviving keys deserialize correctly.
      assert_eq!(parsed.favorite_team.as_deref(), Some("Giants"));
      // Removed keys are silently ignored — no panic, no error.
  }
  ```

- [ ] **Step 2:** If the `toml` crate is not yet a dev-dependency, add it.

  Run: `grep -A 20 "\[dev-dependencies\]" Cargo.toml | grep -q "^toml " || echo "MISSING: add toml = \"0.8\" to [dev-dependencies] in Cargo.toml"`
  If the message prints, add `toml = "0.8"` (matching upstream's version if upstream pins one) under `[dev-dependencies]` in `Cargo.toml`.

---

## Phase 4: Migrate `components/` callsites

Each task in this phase resolves one file's merge conflict (if any), removes `&Symbols` parameters, swaps `Theme::*` references to `palette::*`, swaps `crate::components::util::{era_color, avg_color, ...}` imports to `crate::ui::palette`, and replaces `symbols.nerd_fonts()` / `symbols.team_colors()` boolean branches with the always-on (true) branch (deleting the false branch).

**Working pattern for each callsite-migration task:**

1. Open the file. If it has conflict markers from Phase 1, accept upstream's structural changes (signature shape, `&Symbols` removal) and layer our color/glyph logic back onto it.
2. Replace `use crate::symbols::Symbols;` → delete (no longer needed) or replace with specific function imports like `use crate::symbols::{scoring_play, selection_cursor};` if free functions are called.
3. Replace `use crate::theme::Theme;` → `use crate::ui::palette;` (or `use crate::ui::palette::{EXCELLENT, GOOD, BELOW_AVG, POOR, DIMMED};` for the constants you actually use).
4. Replace `use crate::components::util::{era_color, avg_color, win_pct_color, obp_color, slg_color, ops_color, whip_color, DimColor, convert_color};` → import from new homes:
   - `era_color`/`avg_color`/`win_pct_color`/`obp_color`/`slg_color`/`ops_color`/`whip_color` → `crate::ui::palette::*`
   - `convert_color` → `crate::ui::styling::convert_color`
   - `DimColor` (Color-returning trait): we are *not* preserving this. Replace `value.dim_or(some_color)` callsites with explicit `if value == "0" { palette::DIMMED } else { some_color }` inline. There are ≤4 callsites — boxscore.rs and linescore.rs.
5. Replace `Theme::EXCELLENT` → `palette::EXCELLENT`, similarly for `GOOD`, `BELOW_AVG`, `POOR`, `DIMMED`, `POSITIVE`, `ACCENT_BG`, `ACCENT_FG`, `BORDER`, `TITLE_BG`, `TITLE_FG`.
6. Replace `symbols.nerd_fonts()` → `true` (or simplify by inlining the always-true branch).
7. Replace `symbols.team_colors()` → `true` (same).
8. Replace `symbols.theme().border()` → `palette::border_color()`, similarly `dimmed()` → `palette::dimmed_color()`, `selection_style()` → `palette::selection_style()`, `title_style()` → `palette::title_style()`, `stat_style(c)` → `palette::stat_style(c)`.
9. Replace `symbols.theme().use_backgrounds()` → `true` (rainbow tier collapsed to always-on; this means schedule and standings now always render their backgrounds).
10. Replace `symbols.scoring_play()` → `crate::symbols::scoring_play()`, similarly for every other `symbols.*` glyph call.
11. Remove the `&Symbols` (or `&'a Symbols`, `symbols: &Symbols`) parameter from any function or struct field whose only use of it was the now-removed glyph/theme branches.
12. Run `cargo build --workspace --locked 2>&1 | head -40` after each file. Errors should reduce monotonically — if a fix produces errors elsewhere that you didn't expect, it usually means another file still imports something you just removed.

When the file compiles cleanly in isolation (no errors *originating* from that file), move to the next task.

### Task 4.1: `src/components/util.rs` [checkpoint]

**Files:**
- Modify: `src/components/util.rs`

- [ ] **Step 1:** Resolve the conflict. Final shape: keep our `last_name`, `OptionDisplayExt`, `OptionMapDisplayExt` (and their tests). Accept upstream's visibility flips (`pub(crate)` → `pub` on the survivors). Remove everything else (the `DimColor` trait, all `*_color` functions, `convert_color`, all theme-dependent code, the `stat_color_tests` module).

  The final file content is:

  ```rust
  /// Display an `Option<T>` as a string, using a default if `None`.
  /// e.g. `bio.height.display_or("-")`
  pub trait OptionDisplayExt {
      fn display_or(&self, default: &str) -> String;
  }

  impl<T: std::fmt::Display> OptionDisplayExt for Option<T> {
      fn display_or(&self, default: &str) -> String {
          self.as_ref()
              .map(|v| v.to_string())
              .unwrap_or_else(|| default.to_string())
      }
  }

  /// Map an `Option<T>` through a function, then display as a string with a default if `None`.
  pub trait OptionMapDisplayExt<T> {
      fn map_display_or<U: std::fmt::Display, F: FnOnce(&T) -> U>(
          &self,
          f: F,
          default: &str,
      ) -> String;
  }

  impl<T> OptionMapDisplayExt<T> for Option<T> {
      fn map_display_or<U: std::fmt::Display, F: FnOnce(&T) -> U>(
          &self,
          f: F,
          default: &str,
      ) -> String {
          self.as_ref()
              .map(f)
              .map(|v| v.to_string())
              .unwrap_or_else(|| default.to_string())
      }
  }

  /// Surname for compact display. Skips trailing generational suffixes so "Vladimir Guerrero Jr."
  /// returns "Guerrero" instead of "Jr."
  pub fn last_name(full: &str) -> &str {
      let mut parts = full.rsplitn(3, ' ');
      let tail = parts.next().unwrap_or(full);
      if matches!(tail, "Jr." | "Sr." | "II" | "III" | "IV") {
          parts.next().unwrap_or(tail)
      } else {
          tail
      }
  }

  #[test]
  fn test_last_name() {
      assert_eq!(last_name("Jack Flaherty"), "Flaherty");
      assert_eq!(last_name("J.P. France"), "France");
      assert_eq!(last_name("Vladimir Guerrero Jr."), "Guerrero");
      assert_eq!(last_name("Cal Ripken Jr."), "Ripken");
      assert_eq!(last_name("Ken Griffey Sr."), "Griffey");
      assert_eq!(last_name("Cal Ripken III"), "Ripken");
      assert_eq!(last_name("Robert Person II"), "Person");
      assert_eq!(last_name("Madison"), "Madison");
      assert_eq!(last_name(""), "");
      assert_eq!(last_name("Jr."), "Jr.");
  }
  ```

- [ ] **Step 2:** Stage the file.

  Run: `git add src/components/util.rs`

### Task 4.2: `src/components/standings.rs` [checkpoint]

**Files:**
- Modify: `src/components/standings.rs`

- [ ] **Step 1:** Resolve the conflict using the working pattern at the top of Phase 4. Specifically:
  - Drop `use crate::components::team_colors;`
  - Replace `use crate::components::util::win_pct_color;` → `use crate::ui::palette;`
  - Drop `use crate::symbols::Symbols;` if present.
  - In `to_cells`: change signature from `pub fn to_cells(&self, symbols: &crate::symbols::Symbols) -> Vec<Cell<'_>>` → `pub fn to_cells(&self) -> Vec<Cell<'_>>`. Inside, remove `let theme = symbols.theme();`. Replace `symbols.team_colors()` checks with the always-on branch (delete the else). Replace `theme.<method>()` calls with `palette::<corresponding-fn>()` or direct constant access.
  - Replace `win_pct_color(...)` → `palette::win_pct_color(...)`.

- [ ] **Step 2:** Build and confirm `src/components/standings.rs` produces no errors.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/components/standings.rs" | head -20`
  Expected: no output.

- [ ] **Step 3:** Stage.

  Run: `git add src/components/standings.rs`

### Task 4.3: `src/components/linescore.rs` [continuation]

**Files:**
- Modify: `src/components/linescore.rs`

- [ ] **Step 1:** Apply the working pattern. Specifically:
  - Drop `use crate::symbols::Symbols;`.
  - In `create_score_vec`: change signature from `pub fn create_score_vec(&self, won: bool, symbols: &Symbols) -> Vec<Cell<'_>>` → `pub fn create_score_vec(&self, won: bool) -> Vec<Cell<'_>>`. Inside, replace `symbols.team_colors()` branch with always-on; replace any `Theme::*` with `palette::*`.
  - Replace `use crate::components::util::DimColor;` → remove. Inline any `value.dim_or(color)` calls as `if value == "0" { palette::DIMMED } else { color }` — use `&str` comparison if the value is a `String` (`value.as_str() == "0"`). For `u8`/`u16`, compare `value == 0`.

- [ ] **Step 2:** Build and confirm.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/components/linescore.rs" | head -20`
  Expected: no output.

- [ ] **Step 3:** Stage.

  Run: `git add src/components/linescore.rs`

### Task 4.4: `src/components/stats/player_profile.rs` [continuation]

**Files:**
- Modify: `src/components/stats/player_profile.rs`

- [ ] **Step 1:** Apply the working pattern. Specifically:
  - Drop `use crate::symbols::Symbols;` and `use crate::theme::Theme;`.
  - Update import: `use crate::components::util::{...};` → keep `OptionDisplayExt`, `OptionMapDisplayExt`; remove any color-fn imports.
  - Add: `use crate::ui::palette;`.
  - In `game_log_cells`: change signature from `fn game_log_cells<'a>(split: &'a Split, symbols: &Symbols) -> Vec<Cell<'a>>` → `fn game_log_cells<'a>(split: &'a Split) -> Vec<Cell<'a>>`. Inside, replace `symbols.team_colors()` always-on; replace `Theme::GOOD` / `Theme::DIMMED` with `palette::GOOD` / `palette::DIMMED`.
  - In any function with `symbols: &Symbols` parameter (line ~341 area), drop the parameter and propagate.

- [ ] **Step 2:** Build and confirm.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/components/stats/player_profile.rs" | head -20`
  Expected: no output.

- [ ] **Step 3:** Stage.

  Run: `git add src/components/stats/player_profile.rs`

### Task 4.5: `src/components/probable_pitchers.rs` [continuation]

**Files:**
- Modify: `src/components/probable_pitchers.rs`

- [ ] **Step 1:** Apply pattern. Specifically:
  - `use crate::components::util::{OptionDisplayExt, era_color};` → `use crate::components::util::OptionDisplayExt; use crate::ui::palette;`.
  - Replace `era_color(...)` → `palette::era_color(...)`.
  - Keep our `to_row_cells -> Vec<Cell>` signature change from `feat/colors`.

- [ ] **Step 2:** Build and confirm. Stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/components/probable_pitchers.rs" | head -20 && git add src/components/probable_pitchers.rs`
  Expected: no compile errors output; staged.

### Task 4.6: `src/components/boxscore.rs` [continuation]

**Files:**
- Modify: `src/components/boxscore.rs`

- [ ] **Step 1:** Apply pattern. Specifically:
  - `use crate::components::util::{DimColor, avg_color, era_color};` → `use crate::ui::palette;`.
  - Replace `avg_color(s)` → `palette::avg_color(s)`, `era_color(s)` → `palette::era_color(s)`.
  - For each `value.dim_or(color)` callsite, inline as: `if <value>_is_zero { palette::DIMMED } else { color }`. Determine `_is_zero` by the original type (str → `s == "0"`; u8/u16 → `n == 0`).

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/components/boxscore.rs" | head -20 && git add src/components/boxscore.rs`

### Task 4.7: `src/components/team_page.rs` [continuation]

**Files:**
- Modify: `src/components/team_page.rs`

- [ ] **Step 1:** This file should have no Symbols/Theme usage today (only `OptionDisplayExt`/`OptionMapDisplayExt`). If the merge introduced any Symbols param via upstream, drop it. Keep our `is_win: Option<bool>` field on `TeamGame`.

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/components/team_page.rs" | head -20 && git add src/components/team_page.rs`

### Task 4.8: `src/components/stats/table.rs` [continuation]

**Files:**
- Modify: `src/components/stats/table.rs`

- [ ] **Step 1:** Apply pattern. Specifically:
  - `arrow_symbol(&self, symbols: &crate::symbols::Symbols) -> &'static str` → `arrow_symbol(&self) -> &'static str`. Inside, replace `symbols.sort_asc()` → `crate::symbols::sort_asc()`, `symbols.sort_desc()` → `crate::symbols::sort_desc()`.

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/components/stats/table.rs" | head -20 && git add src/components/stats/table.rs`

### Task 4.9: `src/components/game/matchup.rs` [continuation]

**Files:**
- Modify: `src/components/game/matchup.rs`

- [ ] **Step 1:** Apply pattern. Specifically:
  - Drop `use crate::theme::Theme;` and replace `Theme::BELOW_AVG` / `Theme::DIMMED` references with `palette::BELOW_AVG` / `palette::DIMMED` (add `use crate::ui::palette;`).
  - For functions taking `symbols: &crate::symbols::Symbols`: drop the parameter. Inside, every `symbols.team_colors()` always-on; every `symbols.nerd_fonts()` always-on; every `symbols.base_occupied()` → `crate::symbols::base_occupied()`, `symbols.base_empty()` → `crate::symbols::base_empty()`, etc.
  - In the test module, the `let symbols = Symbols::new(false, false, ThemeLevel::default());` line goes away with the parameter; update the assertion calls to use the new free-function-based code path.

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/components/game/matchup.rs" | head -30 && git add src/components/game/matchup.rs`

### Task 4.10: `src/components/game/pitch_event.rs` [continuation]

**Files:**
- Modify: `src/components/game/pitch_event.rs`

- [ ] **Step 1:** Apply pattern. Specifically:
  - Drop `symbols: &crate::symbols::Symbols` parameter wherever it appears.
  - Replace `symbols.scoring_play()` → `crate::symbols::scoring_play()`.

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/components/game/pitch_event.rs" | head -20 && git add src/components/game/pitch_event.rs`

### Task 4.11: `src/components/game/pitches.rs` and `strikezone.rs` [continuation]

**Files:**
- Modify: `src/components/game/pitches.rs`
- Modify: `src/components/game/strikezone.rs`

- [ ] **Step 1:** In each file, replace `use crate::components::util::convert_color;` → `use crate::ui::styling::convert_color;`. No other changes expected.

- [ ] **Step 2:** Build, confirm, stage both.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/components/game/" | head -20 && git add src/components/game/pitches.rs src/components/game/strikezone.rs`

---

## Phase 5: Migrate `ui/` callsites

Same working pattern as Phase 4. Resolve any merge conflicts in conjunction with the migration.

### Task 5.1: `src/draw.rs` [checkpoint]

**Files:**
- Modify: `src/draw.rs`

- [ ] **Step 1:** Resolve conflict. Specifically:
  - Drop `use crate::symbols::Symbols;`.
  - Accept upstream's signature changes for `draw_*` functions (no `symbols: &Symbols` parameter).
  - Accept upstream's `default_border()` no-arg signature.
  - Where upstream uses `border_style()` from `crate::ui::styling`: that's correct for upstream's chrome. We want our richer `palette::border_color()`. Add `use crate::ui::palette;` and replace `border_style()` calls inside chrome with `Style::default().fg(palette::border_color())` (or define a local helper).
  - Replace any `symbols.format_weather(condition, temp)` → `crate::symbols::format_weather(condition, temp)`.
  - Replace any `symbols.theme().border()` → `palette::border_color()`.
  - Inside `draw_scoreboard`, the temperature color lookup that today uses `Theme::temp_color(temp_val)` becomes `palette::temp_color(temp_val)`. Likewise for any wind color.

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/draw.rs" | head -30 && git add src/draw.rs`

### Task 5.2: `src/ui/standings.rs` [continuation]

**Files:**
- Modify: `src/ui/standings.rs`

- [ ] **Step 1:** Resolve conflict. Specifically:
  - Drop `use crate::symbols::Symbols;` and `use crate::theme::Theme;`.
  - Accept upstream's `pub struct StandingsWidget {}` (drop the symbols field).
  - Add `use crate::ui::palette;`.
  - Replace `self.symbols.theme().use_backgrounds()` → `true` (then simplify any `if true { ... }` to just the body).
  - Replace `s.to_cells(self.symbols)` → `s.to_cells()` (matches Task 4.2's signature change).
  - Replace `self.symbols.theme().selection_style()` → `palette::selection_style()`.
  - Replace `self.symbols.theme().title_style()` → `palette::title_style()`.
  - For chrome `header_style()` calls accepting upstream's helper from `styling`, leave them — they use upstream's defaults. If you want our richer headers, replace with `Style::new().bold().underlined().underline_color(palette::dimmed_color())` (matching upstream's structure but with our underline color). Match the existing in-file style — don't introduce a new pattern just here.

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/ui/standings.rs" | head -20 && git add src/ui/standings.rs`

### Task 5.3: `src/ui/stats.rs` [continuation]

**Files:**
- Modify: `src/ui/stats.rs`

- [ ] **Step 1:** Resolve conflict. Specifically:
  - Drop `use crate::symbols::Symbols;` and `use crate::theme::Theme;`.
  - Add `use crate::ui::palette;`.
  - Drop the `symbols` field on `StatsDataWidget` and `StatsOptionsWidget`.
  - Replace `self.symbols.theme().title_style()` → `palette::title_style()`.
  - Replace `self.symbols.theme().selection_style()` → `palette::selection_style()`.
  - Replace `Theme::DIMMED` → `palette::DIMMED`.
  - Replace `state.table.sorting.order.arrow_symbol(self.symbols)` → `state.table.sorting.order.arrow_symbol()` (matches Task 4.8).
  - Replace `let theme = self.symbols.theme();` and downstream `theme.<x>()` calls accordingly.

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/ui/stats.rs" | head -20 && git add src/ui/stats.rs`

### Task 5.4: `src/ui/schedule.rs` [continuation]

**Files:**
- Modify: `src/ui/schedule.rs`

- [ ] **Step 1:** Resolve conflict. Specifically:
  - Drop `use crate::symbols::Symbols;` and `use crate::theme::Theme;`.
  - Add `use crate::ui::palette;`.
  - Drop the `symbols` field.
  - Replace `self.symbols.theme().use_backgrounds()` → `true`.
  - In `format`: drop `symbols: &Symbols` parameter; replace `symbols.favorite_marker()` → `crate::symbols::favorite_marker()`; replace `symbols.team_colors()` always-on; replace `symbols.theme().use_backgrounds()` → `true`.
  - Replace `self.symbols.theme().selection_style()` → `palette::selection_style()`.
  - Replace `self.symbols.theme().title_style()` → `palette::title_style()`.
  - Where `LIVE_GAME_BG` and `FAVORITE_BG` are used (or referenced via `Theme::*`), use `palette::LIVE_GAME_BG` / `palette::FAVORITE_BG`.

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/ui/schedule.rs" | head -20 && git add src/ui/schedule.rs`

### Task 5.5: `src/ui/player_profile.rs` [continuation]

**Files:**
- Modify: `src/ui/player_profile.rs`

- [ ] **Step 1:** Resolve. Same pattern. Drop `Symbols`/`Theme` imports. Add `palette`. Drop the `symbols` field. Replace `self.symbols.theme().*` calls with `palette::*`. Replace `self.symbols.team_colors()` always-on. Replace `PlayerProfile::build_game_log_rows(splits, self.symbols)` → `PlayerProfile::build_game_log_rows(splits)` (matches Task 4.4 signature change). Replace `render_scrollbar(inner, &mut self.state.scroll_state, self.symbols, buf)` → `render_scrollbar(inner, &mut self.state.scroll_state, buf)` (matches Task 5.10 below).

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/ui/player_profile.rs" | head -20 && git add src/ui/player_profile.rs`

### Task 5.6: `src/ui/team_page.rs` [continuation]

**Files:**
- Modify: `src/ui/team_page.rs`

- [ ] **Step 1:** Resolve conflict. Drop `use crate::theme::Theme;`. Add `use crate::ui::palette;`. Replace `Theme::DIMMED` → `palette::DIMMED`, `Theme::ACCENT_BG` → `palette::ACCENT_BG`, `Theme::ACCENT_FG` → `palette::ACCENT_FG`. The `const PAST_STYLE: Style = Style::new().fg(Theme::DIMMED);` becomes `const PAST_STYLE: Style = Style::new().fg(palette::DIMMED);` — note this requires `palette::DIMMED` to be `const`, which it is.

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/ui/team_page.rs" | head -20 && git add src/ui/team_page.rs`

### Task 5.7: `src/ui/decision_pitchers.rs` [continuation]

**Files:**
- Modify: `src/ui/decision_pitchers.rs`

- [ ] **Step 1:** Resolve conflict. Drop `use crate::theme::Theme;` and any util-imports for moved color fns. Add `use crate::ui::palette;`. Replace `Style::default().fg(Theme::DIMMED)` → `Style::default().fg(palette::DIMMED)`.

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/ui/decision_pitchers.rs" | head -20 && git add src/ui/decision_pitchers.rs`

### Task 5.8: `src/ui/gameday/*.rs` (5 files) [continuation]

**Files:**
- Modify: `src/ui/gameday/at_bat.rs`
- Modify: `src/ui/gameday/gameday_widget.rs`
- Modify: `src/ui/gameday/matchup.rs`
- Modify: `src/ui/gameday/plays.rs`
- Modify: `src/ui/gameday/win_probability.rs`

- [ ] **Step 1:** For each file, apply the working pattern:
  - Drop `use crate::symbols::Symbols;` and `use crate::theme::Theme;`.
  - Add `use crate::ui::palette;` where palette consts are referenced.
  - Drop `symbols` fields from widget structs (`GamedayWidget`, `MatchupWidget`, `PlaysWidget`, `WinProbabilityWidget`, `AtBatWidget`).
  - Drop `symbols: &Symbols` / `symbols: &'a Symbols` parameters from helper functions.
  - Replace `symbols.scoring_play()` → `crate::symbols::scoring_play()`, `symbols.selection_cursor()` → `crate::symbols::selection_cursor()`, etc.
  - Replace `symbols.team_colors()` / `symbols.nerd_fonts()` always-on.
  - In `plays.rs`, replace `pub const GREEN: Color = Theme::POSITIVE; pub const BLUE: Color = Theme::EXCELLENT; pub const RED: Color = Theme::POOR;` → `pub const GREEN: Color = palette::POSITIVE; pub const BLUE: Color = palette::EXCELLENT; pub const RED: Color = palette::POOR;`.
  - In `win_probability.rs`, replace `_symbols: &Symbols` / `symbols: &Symbols` parameters; the body's `symbols.team_colors()` becomes always-on.
  - In `gameday_widget.rs`, the inner widget construction (`MatchupWidget { game: ..., symbols: self.symbols }` and friends) — drop the `symbols:` field on every constructed widget.

- [ ] **Step 2:** Build, confirm, stage. Build between each file to avoid losing the trail of which file caused which error.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/ui/gameday/" | head -40 && git add src/ui/gameday/`

### Task 5.9: `src/ui/linescore.rs` [continuation]

**Files:**
- Modify: `src/ui/linescore.rs`

- [ ] **Step 1:** Resolve conflict. Drop `use crate::symbols::Symbols;`. Drop `symbols` field on the widget. Update the `create_score_vec` calls to drop the `self.symbols` argument (matches Task 4.3).

- [ ] **Step 2:** Build, confirm, stage.

  Run: `cargo build --workspace --locked 2>&1 | grep "src/ui/linescore.rs" | head -20 && git add src/ui/linescore.rs`

### Task 5.10: `src/ui/boxscore.rs` and `src/ui/scroll.rs` [continuation]

**Files:**
- Modify: `src/ui/boxscore.rs`
- Modify: `src/ui/scroll.rs`

- [ ] **Step 1:** In `src/ui/scroll.rs`: change `symbols: &crate::symbols::Symbols` parameter on `render_scrollbar` → drop. Inside, replace `symbols.scroll_up()` → `crate::symbols::scroll_up()`, `symbols.scroll_down()` → `crate::symbols::scroll_down()`.

- [ ] **Step 2:** In `src/ui/boxscore.rs`: drop `use crate::symbols::Symbols;`. Drop `symbols` field on the widget. Replace `render_scrollbar(area, &mut self.state.scroll_state, self.symbols, buf)` → `render_scrollbar(area, &mut self.state.scroll_state, buf)` (matches scroll.rs change above).

- [ ] **Step 3:** Build, confirm, stage both.

  Run: `cargo build --workspace --locked 2>&1 | grep -E "src/ui/(boxscore|scroll)\.rs" | head -20 && git add src/ui/boxscore.rs src/ui/scroll.rs`

### Task 5.11: Resolve any straggler conflicts [continuation]

**Files:** any file still showing in `git diff --name-only --diff-filter=U`.

- [ ] **Step 1:** Confirm all conflict markers are resolved.

  Run: `git diff --name-only --diff-filter=U`
  Expected: empty output.

  If any file is still listed, open it, apply the same working pattern, build, stage. Common stragglers: `src/ui/decision_pitchers.rs` if Phase 5.7 was skipped, or any newly-introduced file from upstream that wasn't anticipated.

---

## Phase 6: Verify

### Task 6.1: Whole-workspace build is clean [checkpoint]

- [ ] **Step 1:** Run a clean build.

  Run: `cargo build --workspace --locked 2>&1 | tail -5`
  Expected: `Finished \`dev\` profile [unoptimized + debuginfo] target(s) in <time>`. No errors. Warnings are tolerated only if they are in upstream-owned `src/ui/styling.rs` (none should be).

  If there are errors, **stop and report**. Do not proceed to fmt/clippy/tests until the build is clean.

### Task 6.2: Format check [continuation]

- [ ] **Step 1:** Auto-format.

  Run: `cargo fmt --all`
  Expected: silent.

- [ ] **Step 2:** Verify the format gate.

  Run: `cargo fmt --all -- --check`
  Expected: silent (exit 0).

### Task 6.3: Clippy gate [continuation]

- [ ] **Step 1:** Run clippy.

  Run: `cargo clippy --workspace --all-targets --all-features --locked -- --deny warnings 2>&1 | tail -10`
  Expected: `Finished \`dev\` profile ...` and exit 0.

  If any warnings appear, fix them before proceeding. Common cases:
  - Unused imports: remove.
  - Dead code from removed `symbols` parameters: clippy will flag — remove.
  - `clippy::unnecessary_wraps` if a refactored function no longer needs `Option<Color>`: leave as-is for stat color fns (the API contract is `Option`); fix elsewhere if it triggers.

### Task 6.4: Test gate [continuation]

- [ ] **Step 1:** Run the workspace test suite.

  Run: `cargo test --workspace --all-targets --all-features --locked 2>&1 | tail -20`
  Expected: all tests pass. The 24 stat-color tests in `palette` pass. The matchup/symbols glyph tests pass. The `last_name` test in `util` passes. The new `old_config_with_removed_keys_loads` test in `config` passes.

  If any test fails, **stop and report** — do not delete or modify a failing test.

### Task 6.5: Survey for stale references [continuation]

- [ ] **Step 1:** Search for residual references to removed symbols.

  Run: `grep -rn "use crate::theme\|crate::theme::\|ThemeLevel\|Symbols::new\|Symbols {" src/ 2>&1`
  Expected: no output.

  Run: `grep -rn "components::util::era_color\|components::util::avg_color\|components::util::win_pct_color\|components::util::obp_color\|components::util::slg_color\|components::util::ops_color\|components::util::whip_color\|components::util::DimColor\|components::util::convert_color" src/ 2>&1`
  Expected: no output.

  Run: `grep -rn "symbols.nerd_fonts\|symbols.team_colors\|symbols.theme()" src/ 2>&1`
  Expected: no output.

  If any of these find hits, address before proceeding.

### Task 6.6: Manual smoke test [checkpoint]

- [ ] **Step 1:** Build the release binary.

  Run: `cargo build --release --locked 2>&1 | tail -3`
  Expected: `Finished \`release\` profile`.

- [ ] **Step 2:** Launch and exercise the UI manually. From the repo root:

  Run: `./target/release/mlbtg`

  Verify by direct observation:
  - Tab bar shows Nerd Font icons (` Scoreboard`, ` Gameday`, ` Stats`, ` Standings`).
  - Scoreboard renders games. If a game is live, the row has the live-game background tint. The favorite-team marker (`★ `) appears on the favorite team's row.
  - Press `s` to switch to Stats. Confirm OBP/SLG/OPS/WHIP/AVG/ERA columns render with tier colors. Sort by OPS — descending top-of-table values are the EXCELLENT (deep blue) hue.
  - Open a player profile. Confirm season stats, splits, game log all show tier coloring. Multi-hit games and HR cells are highlighted. W/L color renders.
  - Press `t` to switch to Standings. Confirm win-pct column has tier coloring. Confirm border color is our brown (BORDER), not terminal default.
  - Press a team abbreviation to open Team Page. Confirm `is_win`-driven W/L coloring on schedule cells. Confirm past games dim.
  - Press `?` for help (or however help is bound). Confirm chrome (border, title) is themed.
  - Quit (`q`).

- [ ] **Step 3:** If anything renders wrong, **stop and report** with a screenshot or description. Do not silently fix at this stage — visual regressions need conscious triage.

---

## Phase 7: Finalize

### Task 7.1: Update `CLAUDE.md` [continuation]

**Files:**
- Modify: `CLAUDE.md`

- [ ] **Step 1:** Replace the existing project-blurb sentence:

  > `mlbtg` is Anthony's fork of [mlb-rs/mlbt](https://github.com/mlb-rs/mlbt) — a ratatui terminal UI for MLB's Stats API. The fork layers visual-accessibility additions (color themes, Nerd Font glyphs, team colors, weather) on top of upstream. **All additions are off by default** — without config changes, behavior matches upstream.

  with:

  > `mlbtg` is Anthony's fork of [mlb-rs/mlbt](https://github.com/mlb-rs/mlbt) — a ratatui terminal UI for MLB's Stats API. Upstream is the **frame**; our work is the **layer**. We do not feed PRs back to upstream and we do not gate features behind opt-in flags. Glyphs and color are the product — a Nerd Font terminal is required.

- [ ] **Step 2:** Add two new sections after the existing `## Project` section:

  ```markdown
  ## Design north star

  The aesthetic target is hot-metal-compositor newspaper typography. Color is **signal**, not decoration: the Fangraphs blue→amber→red scale carries stat tier; W/L is green/red; live state has its own glyph. Glyphs are **marks**: ⚾ for live, ⚑ for final, manicules for current at-bat, dagger for ejected. Density over chrome — the stathead reads at a glance.

  ## Frame and layer

  - **Frame** — `src/ui/styling.rs` is upstream-owned. Do not edit. Cherry-pick upstream styling work onto it as fast-forwards.
  - **Layer** — `src/ui/palette.rs` and `src/symbols.rs` are ours. Stat tier colors, weather scales, semantic backgrounds, chrome accents, and glyph lookups live here. Override or shadow frame helpers at the *callsite import line*; do not push our behavior into the frame file.
  ```

- [ ] **Step 3:** Stage.

  Run: `git add CLAUDE.md`

### Task 7.2: Update `README.md` [continuation]

**Files:**
- Modify: `README.md`

- [ ] **Step 1:** Add (or update if a similar section exists) a Requirements line near the top, immediately under the project description / badges:

  ```markdown
  **Requirements:** a Nerd Font terminal (e.g. Iosevka Nerd Font, JetBrainsMono Nerd Font). Color and glyphs are unconditional.
  ```

- [ ] **Step 2:** Stage.

  Run: `git add README.md`

### Task 7.3: Final smoke check [continuation]

- [ ] **Step 1:** Re-run the full CI gate to make sure docs edits didn't break anything (they shouldn't).

  Run: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features --locked -- --deny warnings && cargo test --workspace --all-targets --all-features --locked 2>&1 | tail -5`
  Expected: all three clean.

### Task 7.4: Inspect the staged tree [continuation]

- [ ] **Step 1:** Confirm all expected files are staged.

  Run: `git status`
  Expected: many "Changes to be committed" lines, no unstaged changes, no untracked files (other than possibly `target/`).

- [ ] **Step 2:** Confirm the merge is still in progress.

  Run: `cat .git/MERGE_HEAD`
  Expected: prints `2001996...` (upstream/main HEAD).

  If `.git/MERGE_HEAD` does not exist, **stop and report** — somewhere along the way the merge state was abandoned and we'd be making a non-merge commit.

### Task 7.5: Land the merge commit [checkpoint]

**Files:** none — git operation only.

- [ ] **Step 1:** Stage the commit message to a temp file for HEREDOC-safe handling.

  Run:
  ```
  cat > /tmp/mlbtg-merge-msg.txt <<'EOF'
  Merge upstream v0.4.0 — fork divergence and frame/layer architecture

  Adopts upstream PR #141 (styling refactor) and the v0.4.0 release tag.
  Reorients mlbtg around the frame/layer principle:

    Frame  src/ui/styling.rs (upstream-owned, do not edit)
    Layer  src/ui/palette.rs (new) — Fangraphs stat tiers, weather
           scales, semantic backgrounds, chrome accents, accessor fns
    Layer  src/symbols.rs (rewritten) — Nerd Font glyphs as free fns

  Removed:
   * src/theme.rs entirely (Theme struct, ThemeLevel enum, tier toggle)
   * Symbols struct (replaced by free functions)
   * config keys: theme_level, nerd_fonts, team_colors
   * &Symbols parameter from every draw / render / to_cells call

  Glyphs and rich color are now unconditional. A Nerd Font terminal is
  required. Old configs containing the removed keys are silently
  ignored.

  Spec:  docs/superpowers/specs/2026-05-05-fork-divergence-and-v0.4.0-merge-design.md
  Plan:  docs/superpowers/plans/2026-05-05-fork-divergence-and-v0.4.0-merge.md

  Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
  EOF
  ```

- [ ] **Step 2:** Commit.

  Run: `git commit -F /tmp/mlbtg-merge-msg.txt`
  Expected: commit succeeds, GPG signs (pinentry will prompt; if pinentry can't reach a tty, surface the failure to Anthony — he commits manually).

- [ ] **Step 3:** Confirm.

  Run: `git log -1 --stat | head -15`
  Expected: shows the merge commit on `merge/upstream-v0.4.0` with two parents (`main` HEAD prior to merge, `2001996`). Commit message shows our merge body.

- [ ] **Step 4:** Clean up the temp file.

  Run: `rm /tmp/mlbtg-merge-msg.txt`

### Task 7.6: Fast-forward `main` [checkpoint]

**Files:** none — git operation only.

- [ ] **Step 1:** Switch to `main` and fast-forward to the merge branch.

  Run: `git checkout main && git merge --ff-only merge/upstream-v0.4.0`
  Expected: `Updating <prev>..<merge-sha>` and `Fast-forward`.

- [ ] **Step 2:** Confirm.

  Run: `git log --oneline -3`
  Expected: top commit is the merge; second commit is `e56e181` ("Merge upstream/main: chrono datetime..."); third is older.

### Task 7.7: Push to origin [checkpoint — requires explicit Anthony approval]

**Files:** none — git operation only.

- [ ] **Step 1:** Show what would be pushed.

  Run: `git log origin/main..main --oneline`
  Expected: one new commit (the merge).

- [ ] **Step 2:** **PAUSE — confirm with Anthony before pushing.** Pushing publishes the work. Do not push autonomously.

- [ ] **Step 3:** If Anthony approves: `git push origin main`. Otherwise wait.

- [ ] **Step 4:** After push, optionally delete the now-redundant merge branch.

  Run: `git branch -d merge/upstream-v0.4.0`
  Expected: `Deleted branch merge/upstream-v0.4.0`.

- [ ] **Step 5:** Confirm origin agrees.

  Run: `git fetch origin && git log origin/main..main --oneline && git log main..origin/main --oneline`
  Expected: both empty (we are in sync).

---

## Self-review notes

**Spec coverage check:**
- §1 architecture (frame/layer): Tasks 2.1 (frame verbatim), 2.2 (mod), 2.3 (palette layer), 2.4 (symbols layer). ✓
- §2 components — what dies/survives: Phase 3 (theme.rs delete + config), Phase 4 (component callsites), Phase 5 (ui callsites). ✓
- §3 newspaper-typography doctrine — out of scope for this plan; documented in CLAUDE.md (Task 7.1). ✓
- §4 config & docs: Phase 3 (config), Task 7.1 (CLAUDE.md), Task 7.2 (README.md), Task 1.4 (CHANGELOG.md). ✓
- §5 test posture: Phase 2 (palette tests embedded), Task 3.4 (config silent-ignore test), Task 6.4 (full suite). ✓
- §risks: Risk 1 (era_color path) addressed by Task 6.5 (grep survey). Risk 2 (old config) addressed by Task 3.4. Risk 3 (Symbols touch surface) addressed by Phase 4/5 file-by-file with build between. ✓

**Type/method consistency check:**
- `to_cells()` (no args) — defined in Task 4.2 (Standing), called in Task 5.2 (StandingsWidget). ✓
- `to_row_cells() -> Vec<Cell>` — defined in Task 4.5 (probable_pitchers), preserved from feat/colors. ✓
- `arrow_symbol(&self) -> &'static str` (no Symbols arg) — defined Task 4.8, called Task 5.3. ✓
- `create_score_vec(&self, won: bool) -> Vec<Cell<'_>>` (no Symbols arg) — defined Task 4.3, called Task 5.9. ✓
- `game_log_cells(split) -> Vec<Cell>` (no Symbols arg) — defined Task 4.4, called Task 5.5. ✓
- `render_scrollbar(area, scroll_state, buf)` (no Symbols arg) — defined Task 5.10 (scroll.rs), called Task 5.5 and Task 5.10 (boxscore). ✓
- `palette::DIMMED` is `pub const Color::Rgb(...)` — referenced as `const PAST_STYLE: Style = Style::new().fg(palette::DIMMED);` in Task 5.6. Verified `Color::Rgb(...)` is a const-fn target, and `Style::new().fg(...)` is const. ✓

**Placeholder scan:** No "TBD"/"TODO"/"implement later"/"add appropriate"/"similar to". Every code-changing step has a code block or a precise description with the exact strings to replace. ✓
