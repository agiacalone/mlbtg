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
// Public API surface kept for callsite use. Phase 6 verification noted some
// items are unused at the moment; they remain as the documented mlbtg palette
// contract (see spec).
#[allow(dead_code)]
pub const BORDER: Color = Color::Rgb(80, 73, 69);
pub const POSITIVE: Color = Color::Rgb(152, 151, 26);
pub const TITLE_BG: Color = Color::Rgb(40, 60, 80);
pub const TITLE_FG: Color = Color::Rgb(235, 219, 178);

// --- Semantic backgrounds ---

pub const ROW_HIGHLIGHT: Color = Color::Rgb(55, 55, 60);
pub const FAVORITE_BG: Color = Color::Rgb(60, 48, 20);
pub const LIVE_GAME_BG: Color = Color::Rgb(20, 45, 30);

// --- Chrome convenience accessors ---

#[allow(dead_code)]
pub fn border_color() -> Color {
    BORDER
}

#[allow(dead_code)]
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
