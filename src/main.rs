mod continuum;

use std::error::Error;
use std::io::{self, Read, Write};
use continuum::{ Engine, EngineConfig };

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::new(EngineConfig {
        tick_timeout_ms: 100,
    });

    engine.start();

    loop {
        let mut buffer = String::new();
        print!("> "); 
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        match buffer.trim() {
            "quit" => {
                engine.stop();
                println!("Exiting...");
                break Ok(())
            },
            _ => println!("Unknown command")
        }
    }
}
