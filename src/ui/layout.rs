use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::achievements::ACHIEVEMENTS;
use crate::state::{CodachiState, Mood};
use crate::ui::animations::AnimationState;
use crate::ui::widgets::{SpeechBubble, StatBar};

pub fn draw(frame: &mut Frame, state: &CodachiState, anim: &AnimationState, show_all_achievements: bool) {
    let outer = frame.area();

    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(" C O D A C H I ")
        .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let inner = main_block.inner(outer);
    frame.render_widget(main_block, outer);

    // If showing full achievements list, render that instead of normal view
    if show_all_achievements {
        let achv_lines: Vec<Line> = ACHIEVEMENTS
            .iter()
            .map(|def| {
                let unlocked = state.achievements.iter().any(|a| a.id == def.key);
                let marker = if unlocked { " * " } else { "   " };
                let color = if unlocked { Color::Magenta } else { Color::DarkGray };
                Line::from(Span::styled(format!("{}{}", marker, def.name), Style::default().fg(color)))
            })
            .collect();
        let header = Line::from(Span::styled(
            " Achievements  [A] to go back",
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        ));
        let mut all_lines = vec![header, Line::from("")];
        all_lines.extend(achv_lines);
        frame.render_widget(Paragraph::new(all_lines), inner);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // level & mood header
            Constraint::Min(6),    // pet sprite + speech bubble
            Constraint::Length(3), // stat bars
            Constraint::Length(1), // points & XP
            Constraint::Length(1), // controls
            Constraint::Length(3), // achievements
        ])
        .split(inner);

    // Header: level & mood
    let level_name = match state.pet.level {
        1 => "Egg",
        2 => "Buddy",
        _ => "Elder",
    };
    let mood_str = match state.pet.mood {
        Mood::Happy => "Happy",
        Mood::Excited => "Excited",
        Mood::Grumpy => "Grumpy",
        Mood::Sleepy => "Sleepy",
        Mood::Sick => "Sick",
    };
    let mood_color = match state.pet.mood {
        Mood::Happy => Color::Green,
        Mood::Excited => Color::Yellow,
        Mood::Grumpy => Color::Red,
        Mood::Sleepy => Color::Blue,
        Mood::Sick => Color::Magenta,
    };
    let header = Line::from(vec![
        Span::styled(
            format!(" * Level {} - {} ", state.pet.level, level_name),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("Mood: {}", mood_str),
            Style::default().fg(mood_color),
        ),
    ]);
    frame.render_widget(Paragraph::new(header), chunks[0]);

    // Pet sprite area
    let sprite = anim.get_current_frame(state.pet.level, state.pet.mood);
    let sprite_area = chunks[1];

    // Center the sprite horizontally
    let sprite_x = sprite_area.x + sprite_area.width.saturating_sub(sprite.width) / 2;
    let sprite_y = sprite_area.y + 1;

    for (i, line) in sprite.lines.iter().enumerate() {
        let y = sprite_y + i as u16;
        if y < sprite_area.y + sprite_area.height {
            frame.buffer_mut().set_string(
                sprite_x,
                y,
                line,
                Style::default().fg(Color::Green),
            );
        }
    }

    // Speech bubble (to the right of sprite)
    if let Some(remark) = anim.current_remark() {
        let bubble_x = sprite_x + sprite.width + 1;
        let bubble_area = Rect::new(
            bubble_x,
            sprite_y,
            sprite_area.width.saturating_sub(bubble_x - sprite_area.x),
            3,
        );
        frame.render_widget(SpeechBubble { text: remark }, bubble_area);
    }

    // Stat bars
    let stat_area = chunks[2];
    let stat_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(stat_area);

    frame.render_widget(
        StatBar {
            label: "Health",
            value: state.pet.health,
            max: 100,
            color: Color::Green,
        },
        Rect::new(stat_rows[0].x + 1, stat_rows[0].y, stat_rows[0].width - 2, 1),
    );
    frame.render_widget(
        StatBar {
            label: "Hunger",
            value: state.pet.hunger,
            max: 100,
            color: Color::Yellow,
        },
        Rect::new(stat_rows[1].x + 1, stat_rows[1].y, stat_rows[1].width - 2, 1),
    );
    frame.render_widget(
        StatBar {
            label: "Clean ",
            value: state.pet.cleanliness,
            max: 100,
            color: Color::Cyan,
        },
        Rect::new(stat_rows[2].x + 1, stat_rows[2].y, stat_rows[2].width - 2, 1),
    );

    // Points & XP
    let xp_threshold = match state.pet.level {
        1 => 500,
        2 => 1500,
        _ => 9999,
    };
    let points_line = Line::from(vec![
        Span::styled(
            format!(" Points: {} ", state.economy.points),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw("| "),
        Span::styled(
            format!("XP: {}/{}", state.pet.xp, xp_threshold),
            Style::default().fg(Color::Magenta),
        ),
    ]);
    frame.render_widget(Paragraph::new(points_line), chunks[3]);

    // Controls
    let controls = Line::from(vec![
        Span::styled(" [F]", Style::default().fg(Color::Yellow)),
        Span::raw("eed  "),
        Span::styled("[C]", Style::default().fg(Color::Cyan)),
        Span::raw("lean  "),
        Span::styled("[A]", Style::default().fg(Color::Magenta)),
        Span::raw("chievements  "),
        Span::styled("[Q]", Style::default().fg(Color::Red)),
        Span::raw("uit"),
    ]);
    frame.render_widget(Paragraph::new(controls), chunks[4]);

    // Recent achievements
    let recent: Vec<&str> = state
        .achievements
        .iter()
        .rev()
        .take(3)
        .filter_map(|a| {
            ACHIEVEMENTS
                .iter()
                .find(|def| def.key == a.id)
                .map(|def| def.name)
        })
        .collect();

    if !recent.is_empty() {
        let achv_lines: Vec<Line> = recent
            .iter()
            .map(|name| {
                Line::from(Span::styled(
                    format!("  * {}", name),
                    Style::default().fg(Color::Magenta),
                ))
            })
            .collect();
        frame.render_widget(Paragraph::new(achv_lines), chunks[5]);
    }
}
