mod continuum;

use std::error::Error;
use std::io::{self, Write};
use continuum::{ Engine, EngineConfig, ProducerEntity, ProductType };

#[derive(Debug, Clone)]
struct Product {
    name: String,
    production_quantity: f64,
    value_per_unit: f64,
}

impl ProductType for Product {
    fn name(&self) -> &str {
        &self.name
    }

    fn production_quantity(&self) -> f64 {
        self.production_quantity
    }

    fn set_production_quantity(&mut self, quantity: f64) {
        self.production_quantity = quantity;
    }

    fn value_per_unit(&self) -> f64 {
        self.value_per_unit
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::new(EngineConfig {
        tick_timeout_ms: 50,
    });

    setup_producers(&mut engine);
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
            input => {
                // check if the name of a producer was typed
                match engine.get_producer(input) {
                    Some(_producer) => {
                        println!("Found {} producer", input);
                    },
                    None => {
                        println!("No producer called {} found", input);
                    }
                }
            },
        }
    }
}

fn setup_producers(engine: &mut Engine) {
    let producer = ProducerEntity {
        base_cost: 1.0,
        cost_coefficient: 1.03,
        product: Product { 
            name: "Gold".to_string(),
            production_quantity: 0.01, 
            value_per_unit: 1.0,
        },
        production_time_ms: 1000,
        time_elapsed: 0,
    };
    engine.add_producer(Box::new(producer));
    let producer = ProducerEntity {
        base_cost: 1.0,
        cost_coefficient: 1.04,
        product: Product {
            name: String::from("Wood"),
            production_quantity: 1.0,
            value_per_unit: 2.0,
        },
        production_time_ms: 500,
        time_elapsed: 0,
    };
    engine.add_producer(Box::new(producer));
}