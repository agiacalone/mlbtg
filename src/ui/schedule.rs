use crate::components::schedule::{Record, ScheduleRow, ScheduleState};
use crate::components::standings::Team;
use crate::components::team_colors;
use crate::state::app_state::HomeOrAway;
use crate::ui::palette;
use crate::ui::styling::{border_style, dim_style, header_style};
use tui::prelude::*;
use tui::widgets::{Block, BorderType, Borders, Cell, Padding, Row, Table};

const HEADER: &[&str; 8] = &["away", "", "", "home", "", "", "time", "status"];

pub struct ScheduleWidget {
    pub tz_abbreviation: String,
    pub favorite_team: Option<Team>,
}

impl ScheduleRow {
    pub const ABBREVIATION_WIDTH: u16 = 70;
    pub const ABBREVIATION_COL_WIDTH: u16 = 5;
    pub const NORMAL_COL_WIDTH: u16 = 11;

    fn format_record(record: Option<Record>) -> String {
        record
            .map(|r| r.to_display_string())
            .unwrap_or(Record::default_display_string())
    }

    fn default_score(score: Option<u8>) -> String {
        let s = score
            .map(|s| s.to_string())
            .unwrap_or_else(|| "-".to_string());
        format!("{s:<3}")
    }

    fn get_styles(&self, team: HomeOrAway) -> (Style, Style) {
        let winning_team = self.winning_team();
        let lose_style = dim_style();
        match winning_team {
            Some(winner) if winner == team => (Style::default(), Style::default()),
            None => (Style::default(), Style::default()),
            _ => (lose_style, lose_style),
        }
    }

    fn format(&self, width: u16, favorite_team: Option<Team>) -> Vec<Span<'_>> {
        let (mut away_team_style, mut away_score_style) = self.get_styles(HomeOrAway::Away);
        let (mut home_team_style, mut home_score_style) = self.get_styles(HomeOrAway::Home);

        // Bold live scores
        let is_live = self.home_score.is_some()
            && self.away_score.is_some()
            && !self.game_status.contains("Final");
        if is_live {
            away_score_style = away_score_style.add_modifier(Modifier::BOLD);
            home_score_style = home_score_style.add_modifier(Modifier::BOLD);
            away_team_style = away_team_style.add_modifier(Modifier::BOLD);
            home_team_style = home_team_style.add_modifier(Modifier::BOLD);
        }
        let away_record = Self::format_record(self.away_record);
        let home_record = Self::format_record(self.home_record);

        let is_favorite = favorite_team
            .map(|t| t.id == self.away_team.id || t.id == self.home_team.id)
            .unwrap_or(false);
        let marker = if is_favorite {
            crate::symbols::favorite_marker()
        } else {
            "  "
        };

        let (away_team, home_team) = if width < Self::ABBREVIATION_WIDTH {
            (self.away_team.abbreviation, self.home_team.abbreviation)
        } else {
            (self.away_team.team_name, self.home_team.team_name)
        };

        // Merge team color into base style — team colors always on
        let color_style = |base: Style, abbr: &str| -> Style {
            team_colors::get(abbr, false)
                .map(|c| base.fg(c))
                .unwrap_or(base)
        };

        vec![
            Span::styled(
                format!("{marker}{away_team}"),
                color_style(away_team_style, self.away_team.abbreviation),
            ),
            Span::styled(away_record, away_team_style),
            Span::styled(Self::default_score(self.away_score), away_score_style),
            Span::styled(
                home_team,
                color_style(home_team_style, self.home_team.abbreviation),
            ),
            Span::styled(home_record, home_team_style),
            Span::styled(Self::default_score(self.home_score), home_score_style),
            Span::raw(self.start_time.to_string()),
            Span::raw(self.game_status.to_string()),
        ]
    }
}

impl StatefulWidget for ScheduleWidget {
    type State = ScheduleState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let header_cells = HEADER.iter().enumerate().map(|(i, h)| {
            if i == 6 {
                Cell::from(format!("{} [{}]", *h, self.tz_abbreviation))
            } else {
                Cell::from(*h)
            }
        });

        let header = Row::new(header_cells).height(1).style(header_style());

        let rows = state.schedule.iter().map(|r| {
            let row = Row::new(r.format(area.width, self.favorite_team));
            let is_fav = self
                .favorite_team
                .map(|t| t.id == r.away_team.id || t.id == r.home_team.id)
                .unwrap_or(false);
            let is_live = r.home_score.is_some()
                && r.away_score.is_some()
                && !r.game_status.contains("Final");
            if is_fav {
                row.style(Style::default().bg(palette::FAVORITE_BG))
            } else if is_live {
                row.style(Style::default().bg(palette::LIVE_GAME_BG))
            } else {
                row
            }
        });
        let name_constraint = if area.width < ScheduleRow::ABBREVIATION_WIDTH {
            Constraint::Length(ScheduleRow::ABBREVIATION_COL_WIDTH + 2) // +2 for the always-2-char favorite marker
        } else {
            // dynamically size the team name column to fit the longest name in the schedule.
            // this accommodates longer international team names (e.g. WBC) while staying tight
            // on MLB-only days.
            let max_name_len = state
                .schedule
                .iter()
                .map(|r| r.home_team.team_name.len().max(r.away_team.team_name.len()))
                .max()
                .unwrap_or(ScheduleRow::NORMAL_COL_WIDTH as usize);
            Constraint::Length(
                (max_name_len.max(ScheduleRow::NORMAL_COL_WIDTH as usize) + 2) as u16,
            ) // +2 for the always-2-char favorite marker
        };
        let widths = [
            name_constraint,        // away team name
            Constraint::Length(6),  // away team record
            Constraint::Length(3),  // away score
            name_constraint,        // home team name
            Constraint::Length(6),  // home team record
            Constraint::Length(3),  // home score
            Constraint::Length(12), // game time
            Constraint::Fill(1),    // game status
        ];

        let t = Table::new(rows, widths)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(border_style())
                    .padding(Padding::new(1, 1, 0, 0))
                    .title(Span::styled(
                        state.date_selector.format_date_border_title(),
                        palette::title_style(),
                    )),
            )
            .row_highlight_style(palette::selection_style());

        StatefulWidget::render(t, area, buf, &mut state.state);
    }
}
