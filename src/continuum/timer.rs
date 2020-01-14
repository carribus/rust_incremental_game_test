use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct Timer {
    thread_handle: Option<thread::JoinHandle<()>>,
    state: Arc<Mutex<TimerState>>,
    timeout_ms: u64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TimerState {
    Stopped,
    Running,
}

impl Timer {
    pub fn new(timeout_ms: u64) -> Self {
        Timer {
            thread_handle: None,
            state: Arc::new(Mutex::new(TimerState::Stopped)),
            timeout_ms,
        }
    }

    pub fn start(&mut self) -> Receiver<u64> {
        let thread_state = self.state.clone();
        let (sender, receiver) = channel();
        let thread_timeout = self.timeout_ms;

        self.set_state(TimerState::Running);

        self.thread_handle = Some(thread::spawn(move || {
            loop {
                let state = { *thread_state.lock().unwrap() };

                if state == TimerState::Running {
                    let now = SystemTime::now();

                    thread::sleep(Duration::from_millis(thread_timeout));
                    match sender.send(now.elapsed().unwrap().as_millis() as u64) {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Send ERROR: {}", e);
                            *thread_state.lock().unwrap() = TimerState::Stopped;
                        }
                    }
                } else {
                    break;
                }
            }
        }));

        receiver
    }

    pub fn stop(&mut self) {
        let handle = self.thread_handle.take();

        self.set_state(TimerState::Stopped);

        match handle {
            Some(h) => match h.join() {
                Err(e) => println!("JOIN Error: {:?}", e),
                Ok(_) => println!("Timer thread joined"),
            },
            None => println!("Timer.stop() called but thread_handle == None!"),
        }
    }

    pub fn state(&self) -> TimerState {
        *self.state.lock().unwrap()
    }

    fn set_state(&mut self, state: TimerState) {
        *self.state.lock().unwrap() = state
    }
}