use std::{env, fmt::Display};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{buffer::Buffer, layout::{Constraint, Layout, Rect}, style::{palette::tailwind::{BLUE, SLATE}, Modifier, Style, Stylize}, symbols, text::Line, widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget, Wrap}, DefaultTerminal};

pub struct App {
    is_running: bool,
    env_list: EnvList,
}

impl App {
    pub fn new() -> Self {
        Self {
            is_running: true,
            env_list: EnvList::new(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// the environment list.
pub struct EnvList {
    items: Vec<Environment>,
    state: ListState,
}

impl EnvList {
    fn new() -> Self {
        Self {
            items: get_variables(),
            state: ListState::default(),
        }
    }
}

/// Enviroment struct, containing the key and value.
#[derive(Debug)]
struct Environment {
    key: String,
    value: String,
}

impl Environment {
    /// Create a new struct from key, and string.
    fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

fn get_variables() -> Vec<Environment>{
    let envs = env::vars();
    let mut variables: Vec<Environment> = Vec::new();

    for (key, value) in envs {
        variables.push(Environment::new(key, value));
    }
    variables
}

impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.is_running {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc) =>  self.quit(),
            (_, KeyCode::Char('q')) =>  self.quit(),
            (KeyModifiers::CONTROL, KeyCode::Char('c')) =>  self.quit(),
            (_, KeyCode::Char('h')|KeyCode::Left) =>  self.select_none(),
            (_, KeyCode::Char('l')|KeyCode::Right) =>  self.select_none(),
            (_, KeyCode::Char('k')|KeyCode::Up) =>  self.select_previous(),
            (_, KeyCode::Char('j')|KeyCode::Down) =>  self.select_next(),
            (_, KeyCode::Char('g')|KeyCode::PageUp) =>  self.select_first(),
            (_, KeyCode::Char('G')|KeyCode::PageDown) =>  self.select_last(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.is_running = false;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
            .areas(area);

        let [list_area, item_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Fill(1)
        ])
        .areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Ratatui Environment Reader")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ or 'jk', 'gG' to move, and <Esc>, Ctrl-c or 'q' to quit")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Environment List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(Style::new().fg(SLATE.c100).bg(BLUE.c800))
            .bg(SLATE.c950);

        let items: Vec<ListItem> = self
            .env_list
            .items
            .iter()
            .map(|item| {
                ListItem::from(item.key.clone())
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.env_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let info = if let Some(i) = self.env_list.state.selected() {
            self.env_list.items[i].value.clone()
        } else {
            "Nothing selected".to_string()
        };

        let block = Block::new()
            .title(Line::raw("Value").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(Style::new().fg(SLATE.c100).bg(BLUE.c800))
            .bg(SLATE.c950)
            .padding(Padding::horizontal(1));

        Paragraph::new(info)
            .block(block)
            .fg(SLATE.c200)
            .wrap( Wrap { trim: false })
            .render(area, buf);
    }
}

impl App {
    fn select_none(&mut self) {
        self.env_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.env_list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.env_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.env_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.env_list.state.select_last();
    }
}
