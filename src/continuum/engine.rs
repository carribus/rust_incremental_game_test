use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

use crate::continuum::timer::{Timer, TimerState};

/// Defines the configuration for an instance of `Engine`
#[derive(Debug, Copy, Clone)]
pub struct EngineConfig {
    pub tick_timeout_ms: u64,
}

#[derive(Debug)]
pub struct Engine {
    config: EngineConfig,
    timer: Arc<Mutex<Timer>>,
    timer_handle: Option<JoinHandle<()>>,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        Engine {
            config,
            timer: Arc::new(Mutex::new(Timer::new(config.tick_timeout_ms))),
            timer_handle: None,
        }
    }

    pub fn start(&mut self) {
        let receiver = { self.timer.lock().unwrap().start() };
        let mut counter = 0;
        let thread_timer = self.timer.clone();

        self.timer_handle = Some(
            thread::spawn(move || {
                loop {
                    let state = { thread_timer.lock().unwrap().state() };
                    
                    match state {
                        TimerState::Running => {
                            match receiver.recv() {
                                Ok(elapsed) => println!("[{}] tick received ({})", counter, elapsed),
                                Err(e) => {
                                    println!("Recv Error: {}", e);
                                    break;
                                }
                            }
                            counter += 1;
                        },
                        TimerState::Stopped => break,
                    }
                }
                println!("Engine's timer recv thread ending...");
            })
        );
    }

    pub fn stop(&mut self) {
        if self.timer_state() == TimerState::Running {
            self.timer.lock().unwrap().stop();
        }

        let handle = self.timer_handle.take();
        match handle {
            Some(h) => {
                match h.join() {
                    Err(e) => println!("JOIN Error: {:?}", e),
                    Ok(_) => println!("Engine timer thread joined"),
                }
            },
            None => println!("Engine.stop() called but not timer thread exists"),
        }
    }

    fn timer_state(&self) -> TimerState {
        self.timer.lock().unwrap().state()
    }
}
