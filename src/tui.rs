use ratatui::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Flex, Layout, Margin, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame, Terminal,
};
use std::{
    fs::{self, File},
    io::stderr,
    time::Duration as StdDuration,
};
use tachyonfx::{
    fx::{self, Direction as FxDirection},
    Duration, Effect, EffectRenderer, EffectTimer, Interpolation, Shader,
};
use tui_textarea::{Input, Key, TextArea};

#[derive(Default)]
pub struct Data {
    pub desc: Option<String>,
    pub prompt: Option<String>,
    pub ttyname: Option<String>,
}

pub struct Tui {
    pub data: Data,

    text_area: TextArea<'static>,
    effect: Effect,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            data: Data::default(),
            text_area: Self::text_area(),
            effect: Self::effect(),
        }
    }

    fn text_area() -> TextArea<'static> {
        let mut text_area = TextArea::default();
        text_area.set_cursor_line_style(Style::default());
        text_area.set_placeholder_text("");
        text_area.set_mask_char('*');
        text_area.set_block(Block::bordered());
        text_area
    }

    fn effect() -> Effect {
        fx::parallel(&[
            fx::sweep_in(
                FxDirection::UpToDown,
                50,
                30,
                Color::Blue,
                EffectTimer::from_ms(2000, Interpolation::Linear),
            ),
            fx::prolong_start(2000, fx::coalesce((1000, Interpolation::SineOut))),
        ])
    }

    fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    fn draw_prompt(&mut self, frame: &mut Frame) {
        let area = Self::popup_area(frame.area(), 40, 30);
        frame.render_widget(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(Color::Rgb(100, 100, 100))),
            area,
        );
        let [input_area, rest] =
            Layout::vertical([Constraint::Min(3), Constraint::Percentage(100)]).areas(area.inner(
                Margin {
                    vertical: 1,
                    horizontal: 1,
                },
            ));
        frame.render_widget(&self.text_area, input_area);
        frame.render_widget(
            Paragraph::new("<OK>").left_aligned(),
            rest.inner(Margin {
                vertical: 0,
                horizontal: 3,
            }),
        );
        frame.render_widget(
            Paragraph::new("<Cancel>").right_aligned(),
            rest.inner(Margin {
                vertical: 0,
                horizontal: 3,
            }),
        );
        if self.effect.running() {
            frame.render_effect(&mut self.effect, area, Duration::from_millis(16));
        }
    }

    pub fn get_pin(&mut self) -> anyhow::Result<String> {
        let mut tty = File::options()
            .write(true)
            .read(true)
            .open(self.data.ttyname.as_deref().unwrap_or("/dev/stdout"))?;

        enable_raw_mode()?;

        execute!(tty, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(tty);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| self.draw_prompt(f))?;
            if ratatui::crossterm::event::poll(StdDuration::from_millis(16))? {
                let event: Input = ratatui::crossterm::event::read()?.into();
                match event.key {
                    Key::Esc => break,
                    Key::Enter => {
                        break;
                    }
                    _ => {
                        self.text_area.input(event);
                    }
                }
            }
        }

        // disable_raw_mode()?;
        // let mut tty = File::open(self.data.ttyname.as_deref().unwrap_or("/dev/stdout"))?;
        // execute!(tty, LeaveAlternateScreen)?;

        Ok(self
            .text_area
            .lines()
            .get(0)
            .unwrap_or(&String::new())
            .to_string())
    }
}
