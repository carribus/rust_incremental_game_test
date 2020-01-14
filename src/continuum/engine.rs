use std::ops::Deref;
use std::sync::mpsc::{Receiver};
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

use crate::continuum::timer::{Timer, TimerState};
use crate::continuum::entities::{Producer};

/// Defines the configuration for an instance of `Engine`
#[derive(Debug, Copy, Clone)]
pub struct EngineConfig {
    pub tick_timeout_ms: u64,
}

/// The inner structure of the Engine
#[derive(Debug)]
struct EngineInner {
    config: EngineConfig,
    timer: Arc<Mutex<Timer>>,
    timer_handle: Option<JoinHandle<()>>,
    producers: Vec<Box<dyn Producer>>,
}

impl EngineInner {
    pub fn start_timer(&self) -> Receiver<u128> {
        self.timer.lock().unwrap().start()
    }

    pub fn stop_timer(&self) {
        self.timer.lock().unwrap().stop()
    }

    pub fn timer_state(&self) -> TimerState {
        self.timer.lock().unwrap().state()
    }

    /// This method is called to progress all entities by a tick
    /// A tick is expressed as a unit of elapsed time since the last tick and the `elapsed` value
    /// allows each entity to calculate how much progress it has made since the last tick. 
    pub fn process_tick(&self, elapsed: u128) {
        let _ = self.producers
                    .iter()
                    .map(|p| {
                        p.on_tick(elapsed);
                    })
                    .collect::<()>();
    }

    pub fn add_producer(&mut self, producer: Box<dyn Producer>) {
        self.producers.push(producer);
    }
}

#[derive(Debug)]
pub struct Engine {
    inner: Arc<Mutex<EngineInner>>,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        Engine {
            inner: Arc::new(Mutex::new(EngineInner {
                config,
                timer: Arc::new(Mutex::new(Timer::new(config.tick_timeout_ms))),
                timer_handle: None,
                producers: Vec::new(),
            })),
        }
    }

    pub fn start(&self) {
        let local_self = self.inner.clone();
        let receiver = local_self.lock().unwrap().start_timer();

        self.inner.lock().unwrap().timer_handle = Some(
            thread::spawn(move || {
                loop {
                    let state = local_self.lock().unwrap().timer_state();

                    match state {
                        TimerState::Running => {
                            match receiver.recv() {
                                Ok(elapsed) => {
                                    println!("Tick received {}", elapsed);
                                    local_self.lock().unwrap().process_tick(elapsed);
                                },
                                Err(e) => {
                                    println!("Recv error: {}", e);
                                    break;
                                },
                            }
                        },
                        TimerState::Stopped => break,
                    }
                }
                println!("Engine's timer recv thread ending");
            })
        );
    }

    pub fn stop(&mut self) {
        let mut inner = self.inner.lock().unwrap();

        if inner.timer_state() == TimerState::Running {
            inner.stop_timer();
        }

        let handle = inner.timer_handle.take();
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

    pub fn add_producer(&mut self, producer: Box<dyn Producer>) {
        self.inner.lock().unwrap().add_producer(producer);
    }
}
