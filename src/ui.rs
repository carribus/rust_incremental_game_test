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
    Terminal,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction}
};

pub use crossterm::event::KeyCode;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
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
                if event::poll(Duration::from_millis(250)).unwrap() {
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

    pub fn show_stuff(&mut self) -> Result<(), io::Error> {
        self.terminal.draw(|mut f| {
            let size = f.size();
            Block::default()
                .title("Block")
                .borders(Borders::ALL)
                .render(&mut f, size);
        })
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

