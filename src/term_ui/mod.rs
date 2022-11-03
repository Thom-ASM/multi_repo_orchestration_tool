use std::{
    io::{self, Stderr},
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame, Terminal,
};

use crate::mrot_core::github::GithubWorkflow;

pub struct TuiApp<'a> {
    title: &'a str,
}

impl TuiApp<'_> {
    pub fn new(title: &str) -> TuiApp {
        TuiApp { title: title }
    }
}

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn run_tui_until_user_exit<B: Backend>(
    term: &mut Terminal<B>,
    app: TuiApp,
    tickRate: Duration,
) -> Result<(), Stderr> {
    loop {
        term.draw(|f| draw_ui(f, &app));

        if event::poll(tickRate).unwrap() {
            let event = event::read().expect("failed to read event");

            match event {
                Event::Key(key) => {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }

                Event::Mouse(m) => match m.kind {
                    MouseEventKind::Drag(drag) => {
                        println!("Drag");
                    }

                    MouseEventKind::Down(d) => {
                        println!("down");
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &TuiApp) {
    let size = f.size();

    let bg = Block::default()
        .title(app.title)
        .borders(Borders::ALL)
        .style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        );

    f.render_widget(bg, size);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let sideBar = Block::default().style(
        Style::default()
            .fg(Color::Green)
            .bg(Color::Green)
            .add_modifier(Modifier::ITALIC | Modifier::BOLD),
    );

    f.render_widget(sideBar, layout[0]);

    let mainGraph = Block::default().style(
        Style::default()
            .fg(Color::Green)
            .bg(Color::Red)
            .add_modifier(Modifier::ITALIC | Modifier::BOLD),
    );

    f.render_widget(mainGraph, layout[1]);
}

// impl Widget for GithubWorkflow {
//     fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {}
// }
