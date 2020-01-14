mod continuum;

use std::error::Error;
use std::io::{self, Write};
use continuum::{ Engine, EngineConfig, ProducerEntity, ProductType };

#[derive(Debug, Clone)]
struct GoldProduct {
    name: String,
    production_quantity: f64,
}

impl ProductType for GoldProduct {
    fn name(&self) -> &str {
        &self.name
    }

    fn production_quantity(&self) -> f64 {
        self.production_quantity
    }

    fn set_production_quantity(&mut self, quantity: f64) {
        self.production_quantity = quantity;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::new(EngineConfig {
        tick_timeout_ms: 100,
    });

    let producer = ProducerEntity {
        base_cost: 1.0,
        cost_coefficient: 1.03,
        output: GoldProduct { 
            name: "Gold".to_string(),
            production_quantity: 1.0, 
        },
        production_time_ms: 1000,
        time_elapsed: 0,
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
