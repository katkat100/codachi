use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

pub struct StatBar<'a> {
    pub label: &'a str,
    pub value: i32,
    pub max: i32,
    pub color: Color,
}

impl<'a> Widget for StatBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 10 || area.height < 1 {
            return;
        }
        let label_width = self.label.len() as u16 + 1;
        let pct_width = 5u16; // " 100%"
        let bar_width = area.width.saturating_sub(label_width + pct_width);

        let pct = if self.max > 0 {
            (self.value as f64 / self.max as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let filled = ((bar_width as f64) * pct) as u16;

        // Label
        buf.set_string(area.x, area.y, self.label, Style::default());
        buf.set_string(area.x + self.label.len() as u16, area.y, ":", Style::default());

        // Bar
        let bar_x = area.x + label_width;
        for i in 0..bar_width {
            let ch = if i < filled { "█" } else { "░" };
            let style = if i < filled {
                Style::default().fg(self.color)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            buf.set_string(bar_x + i, area.y, ch, style);
        }

        // Percentage
        let pct_str = format!("{:>3}%", (pct * 100.0) as u32);
        buf.set_string(
            bar_x + bar_width,
            area.y,
            &pct_str,
            Style::default().fg(self.color),
        );
    }
}

pub struct SpeechBubble<'a> {
    pub text: &'a str,
}

impl<'a> Widget for SpeechBubble<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.text.is_empty() || area.width < 6 || area.height < 3 {
            return;
        }
        let max_text_width = (area.width - 4) as usize;
        let display_text = if self.text.len() > max_text_width {
            &self.text[..max_text_width]
        } else {
            self.text
        };
        let bubble_width = display_text.len() + 4;

        let top = format!("┌{}┐", "─".repeat(bubble_width - 2));
        let mid = format!("│ {} │", display_text);
        let bot = format!("└{}┘", "─".repeat(bubble_width - 2));

        buf.set_string(area.x, area.y, &top, Style::default().fg(Color::Yellow));
        buf.set_string(area.x, area.y + 1, &mid, Style::default().fg(Color::Yellow));
        buf.set_string(area.x, area.y + 2, &bot, Style::default().fg(Color::Yellow));
    }
}

pub struct AchievementLine<'a> {
    pub name: &'a str,
}

impl<'a> Widget for AchievementLine<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 1 {
            return;
        }
        let text = format!(" * {} ", self.name);
        buf.set_string(area.x, area.y, &text, Style::default().fg(Color::Magenta));
    }
}
