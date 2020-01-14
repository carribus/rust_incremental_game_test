mod continuum;

use std::error::Error;
use std::io::{self, Read, Write};
use continuum::{ Engine, EngineConfig, ProducerEntity, ProductType };

#[derive(Debug, Clone)]
struct GoldProduct {
    name: String,
}

impl ProductType for GoldProduct {
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::new(EngineConfig {
        tick_timeout_ms: 100,
    });

    let producer = ProducerEntity {
        base_cost: 1.0,
        cost_coefficient: 1.03,
        output: GoldProduct { name: "Gold".to_string() },
        production_time_ms: 1000,
    };
    engine.add_producer(Box::new(producer));

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
