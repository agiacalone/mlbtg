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
