pub mod continuum {
    use std::sync::mpsc::{channel, Receiver};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, SystemTime};
    use std::thread;

    #[derive(Debug)]
    pub struct Engine {
        timer: Timer,
    }

    impl Engine {
        pub fn new() -> Self {
            Engine {
                timer: Timer::new(100),
            }
        }

        pub fn start(&mut self) {
            let receiver = self.timer.start();
            let mut counter = 0;

            while counter < 10 {
                match receiver.recv() {
                    Ok(elapsed) => println!("[{}] tick received ({})", counter, elapsed),
                    Err(e) => {
                        println!("Recv Error: {}", e);
                        break;
                    }
                }
                counter += 1;
            } 

            self.timer.stop();
        }

        pub fn stop(&self) {

        }
    }

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

        pub fn start(&mut self) -> Receiver<u128> {
            let thread_state = self.state.clone();
            let (sender, receiver) = channel();
            let thread_timeout = self.timeout_ms;

            self.set_state(TimerState::Running);

            self.thread_handle = Some(
                thread::spawn(move || {
                    while *thread_state.lock().unwrap() == TimerState::Running {
                        let now = SystemTime::now();
                        
                        thread::sleep(Duration::from_millis(thread_timeout));
                        match sender.send(now.elapsed().unwrap().as_millis()) {
                            Ok(_) => {
                                println!("tick sent");
                            },
                            Err(e) => {
                                println!("Send ERROR: {}", e);
                                *thread_state.lock().unwrap() = TimerState::Stopped;
                            }
                        }
                    }
                })
            );

            receiver
        }

        pub fn stop(&mut self) {
            println!("Timer stopping..");
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

        fn set_state(&mut self, state: TimerState) {
            *self.state.lock().unwrap() = state
        }
    }
}

fn main() {
    let mut engine = continuum::Engine::new();

    engine.start();
}
