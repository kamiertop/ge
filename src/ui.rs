use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::app::{App, Mode};

pub fn render(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(4),
        Constraint::Min(8),
        Constraint::Length(3),
    ])
    .areas(area);

    render_header(frame, header, app);
    render_body(frame, body, app);
    render_footer(frame, footer, app);
}

fn render_header(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let title = Line::from(vec![
        Span::styled(
            "ge",
            Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::raw("  Git Emoji Picker"),
    ]);

    let input = match app.mode() {
        Mode::Message => Line::from(vec![
            Span::styled("Message: ", Style::new().fg(Color::DarkGray)),
            Span::raw(app.commit_text()),
            Span::styled(cursor(app), Style::new().fg(Color::Cyan)),
        ]),
        Mode::Confirm => Line::from(vec![
            Span::styled("Commit: ", Style::new().fg(Color::DarkGray)),
            Span::raw(app.pending_message().unwrap_or_default()),
        ]),
        _ => Line::from(vec![
            Span::styled("Search: ", Style::new().fg(Color::DarkGray)),
            Span::raw(app.query()),
            Span::styled(cursor(app), Style::new().fg(Color::Cyan)),
        ]),
    };

    let picked = app.picked_emoji().map_or_else(Line::default, |emoji| {
        Line::from(vec![
            Span::styled("Selected: ", Style::new().fg(Color::DarkGray)),
            Span::raw(format!("{} {} ", emoji.icon, emoji.code)),
            Span::raw(emoji.description_zh),
        ])
    });

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(
        Paragraph::new(vec![title, input, picked]).block(block),
        area,
    );
}

fn render_body(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let [list_area, detail_area] =
        Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]).areas(area);

    render_list(frame, list_area, app);
    render_detail(frame, detail_area, app);
}

fn render_list(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let visible_rows = area.height.saturating_sub(2) as usize;
    let start = list_start(app.selected(), visible_rows, app.filtered().len());
    let end = start.saturating_add(visible_rows).min(app.filtered().len());

    let items = app.filtered()[start..end]
        .iter()
        .enumerate()
        .map(|(offset, emoji_index)| {
            let row = start + offset;
            let emoji = &app.emojis()[*emoji_index];
            let marker = if row == app.selected() { ">" } else { " " };
            let style = if row == app.selected() {
                Style::new().fg(Color::Black).bg(Color::Cyan)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::raw(format!("{marker} {} ", emoji.icon)),
                Span::styled(emoji.code, Style::new().add_modifier(Modifier::BOLD)),
                Span::raw(format!("  {}", emoji.description)),
            ]))
            .style(style)
        });

    let block = Block::default()
        .title(format!(
            " Emojis {}-{}/{} ",
            start.saturating_add(1).min(app.filtered().len()),
            end,
            app.filtered().len()
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(List::new(items).block(block), area);
}

fn list_start(selected: usize, visible_rows: usize, total: usize) -> usize {
    if visible_rows == 0 || total <= visible_rows {
        return 0;
    }

    let half_page = visible_rows / 2;
    let max_start = total - visible_rows;

    selected.saturating_sub(half_page).min(max_start)
}

fn render_detail(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let lines = match app.picked_emoji().or_else(|| app.selected_emoji()) {
        Some(emoji) => vec![
            Line::from(Span::styled(
                emoji.icon,
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )),
            Line::raw(""),
            Line::from(vec![
                Span::styled("Code: ", Color::DarkGray),
                Span::raw(emoji.code),
            ]),
            Line::from(vec![
                Span::styled("Use:  ", Color::DarkGray),
                Span::raw(emoji.description),
            ]),
            Line::from(vec![
                Span::styled("用法: ", Color::DarkGray),
                Span::raw(emoji.description_zh),
            ]),
            Line::raw(""),
            Line::from(Span::styled("Keywords:", Color::DarkGray)),
            Line::raw(emoji.keywords.join(", ")),
        ],
        None => vec![Line::raw("No emoji matched your search.")],
    };

    let block = Block::default()
        .title(" Detail ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(
        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .block(block),
        area,
    );
}

fn render_footer(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let shortcuts = match app.mode() {
        Mode::Browse => "↑/↓ move  / search  Enter select  q quit",
        Mode::Search => "type to filter  ↑/↓ move  Enter select  Esc close  Ctrl-C quit",
        Mode::Message => "type message  Enter confirm  Esc cancel  Ctrl-C quit",
        Mode::Confirm => "Enter commit  Esc edit  Ctrl-C quit",
    };

    let line = Line::from(vec![
        Span::styled(app.message(), Style::new().fg(Color::Green)),
        Span::raw("  "),
        Span::styled(shortcuts, Style::new().fg(Color::DarkGray)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    frame.render_widget(Paragraph::new(line).block(block), area);

    if matches!(app.mode(), Mode::Search | Mode::Message | Mode::Confirm) {
        draw_input_hint(frame, area, app.mode());
    }
}

fn draw_input_hint(frame: &mut Frame<'_>, footer: Rect, mode: Mode) {
    let text = match mode {
        Mode::Search => "Search mode",
        Mode::Message => "Message input",
        Mode::Confirm => "Confirm commit",
        Mode::Browse => "",
    };

    let area = Rect {
        x: footer.x.saturating_add(2),
        y: footer.y.saturating_sub(2),
        width: footer.width.saturating_sub(4).min(36),
        height: 3,
    };
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(text)
            .centered()
            .block(Block::default().borders(Borders::ALL)),
        area,
    );
}

fn cursor(app: &App) -> &'static str {
    match app.mode() {
        Mode::Browse => "",
        Mode::Search | Mode::Message => "█",
        Mode::Confirm => "",
    }
}
