use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::DefaultTerminal;

use crate::emoji::{Emoji, git_emojis};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Browse,
    Search,
    Message,
}

#[derive(Debug)]
pub struct App {
    emojis: Vec<Emoji>,
    filtered: Vec<usize>,
    selected: usize,
    picked: Option<usize>,
    query: String,
    commit_text: String,
    submitted_message: Option<String>,
    mode: Mode,
    should_quit: bool,
    message: String,
}

impl Default for App {
    fn default() -> Self {
        let emojis = git_emojis();
        let filtered = (0..emojis.len()).collect();

        Self {
            emojis,
            filtered,
            selected: 0,
            picked: None,
            query: String::new(),
            commit_text: String::new(),
            submitted_message: None,
            mode: Mode::Browse,
            should_quit: false,
            message: "Use ↑/↓ to move, / to search, Enter to select, q to quit".into(),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| crate::ui::render(frame, self))?;
            self.handle_event()?;
        }

        Ok(())
    }

    pub fn emojis(&self) -> &[Emoji] {
        &self.emojis
    }

    pub fn filtered(&self) -> &[usize] {
        &self.filtered
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn selected_emoji(&self) -> Option<&Emoji> {
        self.filtered
            .get(self.selected)
            .and_then(|index| self.emojis.get(*index))
    }

    pub fn picked_emoji(&self) -> Option<&Emoji> {
        self.picked.and_then(|index| self.emojis.get(index))
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn commit_text(&self) -> &str {
        &self.commit_text
    }

    pub fn submitted_message(&self) -> Option<&str> {
        self.submitted_message.as_deref()
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    fn handle_event(&mut self) -> std::io::Result<()> {
        let Event::Key(key) = event::read()? else {
            return Ok(());
        };

        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match self.mode {
            Mode::Browse => self.handle_browse_key(key),
            Mode::Search => self.handle_search_key(key),
            Mode::Message => self.handle_message_key(key),
        }

        Ok(())
    }

    fn handle_browse_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => self.quit(),
            KeyCode::Char('q') | KeyCode::Esc => self.quit(),
            KeyCode::Char('/') => {
                self.mode = Mode::Search;
                self.message = "Search by emoji code, description, or keyword".into();
            }
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::PageDown => self.select_offset(10),
            KeyCode::PageUp => self.select_offset(-10),
            KeyCode::Home => self.selected = 0,
            KeyCode::End => self.selected = self.filtered.len().saturating_sub(1),
            KeyCode::Enter => self.confirm_selection(),
            _ => {}
        }
    }

    fn handle_search_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => self.quit(),
            KeyCode::Esc => {
                self.mode = Mode::Browse;
                self.message = "Search closed".into();
            }
            KeyCode::Enter => {
                self.confirm_selection();
            }
            KeyCode::Backspace => {
                self.query.pop();
                self.apply_filter();
            }
            KeyCode::Char(ch) => {
                self.query.push(ch);
                self.apply_filter();
            }
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            _ => {}
        }
    }

    fn handle_message_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => self.quit(),
            KeyCode::Esc => {
                self.mode = Mode::Browse;
                self.picked = None;
                self.message = "Message input canceled".into();
            }
            KeyCode::Enter => self.submit_message(),
            KeyCode::Backspace => {
                self.commit_text.pop();
            }
            KeyCode::Char(ch) => {
                self.commit_text.push(ch);
            }
            _ => {}
        }
    }

    fn apply_filter(&mut self) {
        let query = self.query.trim().to_lowercase();

        self.filtered = self
            .emojis
            .iter()
            .enumerate()
            .filter_map(|(index, emoji)| emoji.matches(&query).then_some(index))
            .collect();

        self.selected = self.selected.min(self.filtered.len().saturating_sub(1));
        self.message = match self.filtered.len() {
            0 => "No matching git emoji".into(),
            1 => "1 match".into(),
            count => format!("{count} matches"),
        };
    }

    fn select_next(&mut self) {
        if self.filtered.is_empty() {
            return;
        }

        self.selected = (self.selected + 1).min(self.filtered.len() - 1);
    }

    fn select_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    fn select_offset(&mut self, offset: isize) {
        if self.filtered.is_empty() {
            return;
        }

        self.selected = self
            .selected
            .saturating_add_signed(offset)
            .min(self.filtered.len() - 1);
    }

    fn confirm_selection(&mut self) {
        if let Some(index) = self.filtered.get(self.selected).copied() {
            let emoji = &self.emojis[index];
            self.picked = Some(index);
            self.commit_text.clear();
            self.mode = Mode::Message;
            self.message = format!(
                "Selected {} {}. Type commit message.",
                emoji.icon, emoji.code
            );
        }
    }

    fn submit_message(&mut self) {
        let text = self.commit_text.trim();

        if text.is_empty() {
            self.message = "Commit message cannot be empty".into();
            return;
        }

        if let Some(emoji) = self.picked_emoji() {
            self.submitted_message = Some(format!("{} {}", emoji.code, text));
            self.should_quit = true;
        }
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}
