mod continuum;

use std::error::Error;
use std::io::{self, Write};
use continuum::{ Engine, EngineConfig, ProducerEntity, ProductType };

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
                    Some(producer) => {
                        println!("Found {} producer", input);
                        let mut producer = producer.lock().unwrap();
                        let q = producer.production_quantity();
                        
                        producer.set_production_quantity(q * 2.0);
                        
                        println!("gold production per tick is {}", producer.production_quantity());
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
        id: "gold".to_string(),
        base_cost: 1.0,
        cost_coefficient: 1.03,
        product_type: ProductType { 
            name: "Gold".to_string(),
            production_quantity: 0.01, 
            value_per_unit: 1.0,
        },
        production_time_ms: 1000,
        time_elapsed: 0,
    };
    engine.add_producer(Box::new(producer));
    let producer = ProducerEntity {
        id: "wood".to_string(),
        base_cost: 1.0,
        cost_coefficient: 1.04,
        product_type: ProductType {
            name: "Wood".to_string(),
            production_quantity: 1.0,
            value_per_unit: 2.0,
        },
        production_time_ms: 500,
        time_elapsed: 0,
    };
    engine.add_producer(Box::new(producer));
}