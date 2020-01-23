use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Receiver};
use crossterm::{
    event::{self, Event as CEvent, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    Frame,
    Terminal,
    widgets::{Widget, Block, Borders, Paragraph, Text, List, SelectableList, Table, Row},
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    style::{Style, Color},
};
use crate::continuum::{Engine};

const GAME_TITLE: &str = "[ Idle Terminal ]";

pub use crossterm::event::KeyCode;

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
                    if let CEvent::Key(key) = event::read().unwrap() {
                        tx.send(Event::Input(key)).unwrap();
                    }
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

            // Block::default()
            //     .title(GAME_TITLE)
            //     .borders(Borders::ALL)
            //     .render(&mut f, chunks[0]);

            // Block::default()
            //     .borders(Borders::ALL)
            //     .render(&mut f, chunks[1]);
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

        // for (idx, (k, v)) in products.iter().enumerate() {
        //     Paragraph::new([Text::raw(format!("{}: {} | ", k, v))].iter())
        //         .block(
        //             Block::default()
        //                 .borders(Borders::ALL)
        //                 .title("Product")
        //         )
        //         .render(f, chunks[idx]);
        // }

        // let text_vec = products.iter().map(|(k, v)| {
        //     Paragraph::new([Text::raw(format!("{}: {} | ", k, v))].iter())
        //         .block(
        //             Block::default()
        //                 .borders(Borders::ALL)
        //         )
        //         .render(f, area)
        // }).collect::<Vec<Paragraph<Text>>>();

        // Paragraph::new(text_vec.iter())
        //     .block(Block::default()
        //         .borders(Borders::ALL)
        //     )
        //     .alignment(Alignment::Left)
        //     .wrap(true)
        //     .render(f, area);
    }

    fn render_body(f: &mut Frame<TerminalBackend>, engine: &Engine, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(25), Constraint::Min(0)].as_ref())
            .split(area);

        Self::render_products(f, engine, chunks[0]);
    }

    fn render_products(f: &mut Frame<TerminalBackend>, engine: &Engine, area: Rect) {
        let products = engine.get_products();
        let text_vec = products.iter().map(|(k, v)| {
            (k.clone(), format!("{}", v))
        }).collect::<Vec<_>>();

        // TODO: You left off here
        Table::new(
            ["Item", "Qty"].into_iter(),
            text_vec.iter().map(|(label, value)| {
                Row::Data([label, value].into_iter())
            })
        )
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Products")
        )
        .header_style(Style::default().fg(Color::Yellow));
        
        // SelectableList::default()
        //     .block(Block::default()
        //         .borders(Borders::ALL)
        //         .title("Products")
        //     )
        //     .items(&text_vec)
        //     .render(f, area);
    }
}


impl Drop for UI {
    fn drop(&mut self) {
        println!("UI dropping");
        disable_raw_mode().unwrap();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        self.terminal.show_cursor().unwrap();
    }
}

