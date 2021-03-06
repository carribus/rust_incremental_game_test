use std::sync::mpsc::{Receiver};
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

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
    producers: Vec<Arc<Mutex<Box<dyn Producer>>>>,
    products: HashMap<String, f64>,
}

impl EngineInner {
    pub fn start_timer(&self) -> Receiver<u64> {
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
    pub fn process_tick(&mut self, elapsed: u64) {
        let iter = { self.producers.iter_mut() };
        for producer in iter {
            let mut producer = producer.lock().unwrap();
            let q = producer.on_tick(elapsed);
            
            // if anything was produced, allocate it to the relevant production 'bucket'
            if q > 0.0 {
                let product_name = producer.product_type().name.to_string();
                // println!("{} {} produced...", q, product_name);
                // now we need to do something with what was produced...
                let pq = {
                    match self.products.get(&product_name) {
                        Some(v) => *v,
                        None => 0.0,
                    }
                };
                self.products.insert(product_name, pq + q);
                // println!("{:?}", self.products);
            }  
        }           
    }

    pub fn add_producer(&mut self, producer: Box<dyn Producer>) {
        self.producers.push(Arc::new(Mutex::new(producer)));
    }

    pub fn get_producer(&self, id: &str) -> Option<Arc<Mutex<Box<dyn Producer>>>> {
        let mut result = None;

        for p in self.producers.iter() {
            if p.lock().unwrap().id() == id {
                result = Some(p.clone());
                break;
            }
        }

        result
    }

    pub fn get_products(&self) -> HashMap<String, f64> {
        self.products.clone()
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
                products: HashMap::new(),
            })),
        }
    }

    pub fn start(&self) {
        let local_self = self.inner.clone();
        let receiver = local_self.lock().unwrap().start_timer();

        self.inner.lock().unwrap().timer_handle = Some(
            thread::spawn(move || {
                loop {
                    let state = { local_self.lock().unwrap().timer_state() };

                    match state {
                        TimerState::Running => {
                            match receiver.recv() {
                                Ok(elapsed) => {
                                    // println!("Tick received {}", elapsed);
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
                println!("Engine's time recv thread terminating");
            })
        );
    }

    pub fn stop(&mut self) {
        let handle = {
            let mut inner = self.inner.lock().unwrap();

            if inner.timer_state() == TimerState::Running {
                inner.stop_timer();
            }
    
            inner.timer_handle.take()
        };

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

    /// Add a producer to the engine.
    /// Once a producer has been added to the engine and the timer is in a state of `TimerState::Running`,
    /// the producer will receive calls to its `on_tick()` method for processing
    pub fn add_producer(&mut self, producer: Box<dyn Producer>) {
        self.inner.lock().unwrap().add_producer(producer);
    }

    pub fn get_producer(&self, id: &str) -> Option<Arc<Mutex<Box<dyn Producer>>>> {
        self.inner.lock().unwrap().get_producer(id)
    }

    pub fn get_products(&self) -> HashMap<String, f64> {
        self.inner.lock().unwrap().get_products()
    }
}
