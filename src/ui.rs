use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Receiver};
use crossterm::{
    event::{self, Event as CEvent, KeyEvent, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    Frame,
    Terminal,
    widgets::{Widget, Block, Borders, Paragraph, Text, List, SelectableList, Table, Row},
    layout::{Layout, Constraint, Direction, Rect, Alignment, Margin},
    style::{Style, Color},
};
use crate::continuum::{Engine};
use crate::custom_widgets::{Label, Button};

const GAME_TITLE: &str = "[ Idle Terminal ]";

pub use crossterm::event::{KeyCode};

type TerminalBackend = CrosstermBackend<io::Stdout>;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct UI {
    terminal: Terminal<TerminalBackend>,
    event_receiver: Receiver<Event<KeyEvent>>,
}

impl UI {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        enable_raw_mode()?;

        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        execute!(stdout, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.hide_cursor()?;
        terminal.clear()?;

        // start input handling thread
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no events, sent tick event.
                if event::poll(Duration::from_millis(100)).unwrap() {
                    match event::read().unwrap() {
                        CEvent::Key(key) => tx.send(Event::Input(key)).unwrap(),
                        _ => (),
                    }
                    // if let CEvent::Key(key) = event::read().unwrap() {
                    //     tx.send(Event::Input(key)).unwrap();
                    // }
                }

                tx.send(Event::Tick).unwrap();
            }
        });

        Ok(UI{
            terminal,
            event_receiver: rx,
        })
    }

    pub fn event_receiver(&self) -> &Receiver<Event<KeyEvent>> {
        &self.event_receiver
    }

    pub fn render(&mut self, engine: &Engine) -> Result<(), io::Error> {
        self.terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
                .split(size);
            
            Self::render_top_bar(&mut f, engine, chunks[0]);
            Self::render_body(&mut f, engine, chunks[1]);
        })
    }

    fn render_top_bar(f: &mut Frame<TerminalBackend>, engine: &Engine, area: Rect) {
        Paragraph::new([Text::raw("\nWelcome to the Terminal Idle Game - where not even the develop knows whats going to happen...")].iter())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(GAME_TITLE)
                    .borders(Borders::ALL)
            )
            .render(f, area);
    }

    fn render_body(f: &mut Frame<TerminalBackend>, engine: &Engine, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(50), Constraint::Min(0)].as_ref())
            .split(area);

        Self::render_products(f, engine, chunks[0]);
        Self::render_actions(f, engine, chunks[1]);
    }

    fn render_products(f: &mut Frame<TerminalBackend>, engine: &Engine, area: Rect) {
        let products = engine.get_products();
        let text_vec = products.iter().map(|(k, v)| {
            (k.clone(), format!("{}", v))
        }).collect::<Vec<_>>();

        if text_vec.len() > 0 {
            Table::new(
                ["Item", "Qty"].into_iter(),
                text_vec.iter().map(|(label, value)| {
                    Row::Data(vec![label, value].into_iter())
                })
            )
            .block(Block::default().title("[ Products ]").borders(Borders::ALL))
            .header_style(Style::default().fg(Color::Cyan))
            .widths(&[Constraint::Length(9), Constraint::Min(0)])
            .style(Style::default().fg(Color::White))
            .column_spacing(1)
            .render(f, area);
        } else {
            Block::default()
                .borders(Borders::ALL)
                .title("[ Products ]")
                .render(f, area);
        }
    }

    fn render_actions(f: &mut Frame<TerminalBackend>, engine: &Engine, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(0)].as_ref())
            .split(area);

        Block::default()
            .borders(Borders::ALL)
            .title("[ Actions ]")
            .render(f, chunks[0]);

        // TODO: render some buttons here
        let inner_rect = Rect {
            x: chunks[0].x + 1,
            y: chunks[0].y + 1,
            width: chunks[0].width-2,
            height: chunks[0].height-2,
        };

        let btn_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(1,3), Constraint::Ratio(1, 3)].as_ref())
            .split(inner_rect);

        // println!("btn_chunks = {:?}", btn_chunks);

        Button::default()
            .text("[A] Gold Miner")
            .style(Style::default().fg(Color::White).bg(Color::Cyan))
            .render(f, btn_chunks[0]);

        Button::default()
            .text("[B] Wood Cutter")
            .style(Style::default().fg(Color::Black).bg(Color::Yellow))
            .render(f, btn_chunks[1]);

        Block::default()
            .borders(Borders::ALL)
            .title("[ Game Log ]")
            .render(f, chunks[1]);
    }
}


impl Drop for UI {
    fn drop(&mut self) {
        println!("UI dropping");
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        execute!(self.terminal.backend_mut(), DisableMouseCapture).unwrap();
        self.terminal.show_cursor().unwrap();
    }
}

