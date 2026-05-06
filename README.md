# mlbtg

A fork of [mlbt](https://github.com/mlb-rs/mlbt) focused on readability and
visual accessibility.

[![Built With Ratatui](https://ratatui.rs/built-with-ratatui/badge.svg)](https://ratatui.rs/)

MLB in your terminal — gameday, scores, stats, standings, teams, and player
profiles. Powered by MLB's public Stats API, browse today's games or dig through
decades of historical data.

**Requirements:** a Nerd Font terminal (e.g. Iosevka Nerd Font, JetBrainsMono Nerd Font). Color and glyphs are unconditional.

## Why this fork?

Upstream is the **frame**; this fork is the **layer**. The aesthetic target is
hot-metal-compositor newspaper typography — density over chrome, glyphs as
marks, color as signal:

- **Fangraphs-style stat tiers** — blue→amber→red coloring on stat cells so
  outliers jump off the screen.
- **Nerd Font glyphs** — weather icons, base runner diamonds, play-by-play
  labels (K, BB, 2B, HR), live-game marks, tab icons.
- **Team colors** on names in the scoreboard and standings.
- **Weather conditions** shown in the gameday matchup and scoreboard.

Glyphs and rich color are unconditional — a Nerd Font terminal is required.

### Scoreboard

![Scoreboard showing team colors, stat backgrounds, and weather](images/scoreboard-rainbow-large.png)

### Gameday

![Gameday view showing play-by-play labels, strike zone, and weather](images/gameday-rainbow-small.png)

## Installation

### Build from source

```bash
git clone https://github.com/agiacalone/mlbtg.git
cd mlbtg
cargo build --release
```

The binary is at `target/release/mlbtg`. Copy it somewhere on your `$PATH`, or
run directly with `cargo run`.

### Docker

Build and run with Docker:

```bash
docker build -t mlbtg .
docker run -it --rm mlbtg
```

To mount your config file into the container:

```bash
docker run -it --rm -v ~/.config/mlbt:/root/.config/mlbt mlbtg
```

> For the original upstream release binaries, Homebrew tap, and Docker images,
> see [mlb-rs/mlbt](https://github.com/mlb-rs/mlbt#installation).

## Features

- **Scoreboard & box score** — favorite team sorting, full box score, probable
  pitchers, win probability graph, weather for live/completed games, selectable
  date.

- **Gameday** — pitch display, strike zone heat map, selectable at bats, exit
  velocity / launch angle / distance, ABS challenge info (2026+), leverage index
  and win probability per at bat, weather with Nerd Font icons.

- **Stats** — player and team pitching/hitting stats, Fangraphs-inspired stat
  coloring (ERA, AVG, win%), sort by any column, fuzzy search, selectable date.

- **Standings** — favorite team sorting, division/league view, stat coloring
  (win%, streak, run differential), selectable date.

- **Team page** — roster (active and 40-man), schedule with calendar, recent
  transactions.

- **Player profile** — bio, career stats, splits, recent games.

- **Favorite team highlighting** — set your team in the config to sort it first
  in the scoreboard and highlight it in standings.

## Usage

Run `mlbtg` (or `cargo run`) to start. Press `q` to exit.

### Tabs

| Key | Tab |
|-----|-----|
| `1` | Scoreboard |
| `2` | Gameday |
| `3` | Stats |
| `4` | Standings |
| `?` | Help |
| `f` | Toggle full screen |

### Scoreboard

| Key | Description |
|-----|-------------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `Enter` | View game in Gameday |
| `:` | Date picker |
| `w` | Toggle win probability |
| `h` / `a` | Home / away team in box score |
| `Shift` + `j` / `k` | Scroll box score |

### Gameday

Toggle panes:

| Key | Pane |
|-----|------|
| `i` | Info |
| `p` | Pitches |
| `b` | Box score |
| `w` | Win probability |

Navigate at bats:

| Key | Description |
|-----|-------------|
| `j` / `↓` | Previous at bat |
| `k` / `↑` | Next at bat |
| `l` | Jump to live / latest |
| `s` | Jump to first |

### Stats

| Key | Description |
|-----|-------------|
| `←` / `→` / `Tab` | Switch pane (data / options) |
| `j` / `k` | Move up / down |
| `Shift` + `j` / `k` | Page up / down |
| `p` / `h` | Pitching / hitting |
| `t` / `l` | Team / player |
| `a` / `u` | All / qualified players |
| `Ctrl` + `f` | Fuzzy search |
| `o` | Toggle options pane |
| `s` | Sort by selected column |
| `Enter` | View player profile or team page |
| `:` | Date picker |

### Standings

| Key | Description |
|-----|-------------|
| `j` / `k` | Move up / down |
| `Enter` | View team page |
| `l` | Toggle division / league view |
| `:` | Date picker |

### Player Profile

| Key | Description |
|-----|-------------|
| `j` / `k` | Scroll |
| `Shift` + `j` / `k` | Page scroll |
| `s` | Toggle stat category |
| `Esc` | Close |

### Team Page

| Key | Description |
|-----|-------------|
| `←` / `→` / `Tab` | Switch section |
| `j` / `k` | Move up / down |
| `Shift` + `j` / `k` | Page up / down |
| `c` | Toggle calendar |
| `r` | Toggle roster type |
| `Enter` | View player profile |
| `Esc` | Close |

### Date Picker

| Key | Description |
|-----|-------------|
| `←` / `→` | Navigate date |
| `Enter` | Confirm |
| `Esc` | Cancel |
| `today` / `t` | Jump to today |

> Each tab has its own date — viewing historical stats won't change the
> scoreboard date.

## Config

The TUI can be configured two ways:

1. **TUI editor** (recommended): open the help page with `?`, press `Tab` to
   focus the settings panel on the right, and use `j/k` to pick a field. Press
   `Enter` to open the picker and then `Enter` again to save the setting.
2. **Manually editing the config file**: edit the toml file directly. Useful for
   timezones outside the curated options in the TUI.

The config file is located at:

- Linux:   `~/.config/mlbt/mlbt.toml`
- macOS:   `~/Library/Application Support/mlbt/mlbt.toml`
- Windows: `~\AppData\Roaming\mlbt\mlbt.toml`

> The path is also shown at the bottom of the `Help` page.

### Settings

| Setting | Description | Default |
|---------|-------------|---------|
| `favorite_team` | Your team — sorted first in schedule, highlighted in standings. TUI: picker shows all 30 teams. Manually: use the full name (e.g. `"San Francisco Giants"`). | none |
| `timezone` | Game time display. TUI: picker covers common zones. Manually: any value from the [tz database](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones). | `"US/Pacific"` |
| `log_level` | `"off"`, `"trace"`, `"debug"`, `"info"`, `"warn"`, `"error"`. | `"error"` |

### Example

```toml
favorite_team = "San Francisco Giants"
timezone = "US/Pacific"
```

## Acknowledgments

This is a fork of [mlbt](https://github.com/mlb-rs/mlbt) by
[mlb-rs](https://github.com/mlb-rs). Built with
[ratatui](https://github.com/ratatui/ratatui).

## Copyright Notice

Data is supplied by MLB's Stats API and is subject to the license at
http://gdx.mlb.com/components/copyright.txt.

This application and its authors are not affiliated with MLB.

## License

[MIT License](LICENSE)
