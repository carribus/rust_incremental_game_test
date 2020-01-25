mod continuum;
mod custom_widgets;
mod ui;

use continuum::{Engine, EngineConfig, ProducerEntity, ProductType};
use std::error::Error;
use std::io::{self, Write};
use ui::{Event, KeyCode, UI};

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::new(EngineConfig {
        tick_timeout_ms: 50,
    });

    setup_producers(&mut engine);
    engine.start();

    let mut ui = UI::new().unwrap();

    loop {
        ui.render(&engine)?;
        match ui.event_receiver().recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('a') => {
                    add_gold_mine(&mut engine);
                }
                KeyCode::Char('b') => {
                    add_wood_cutter(&mut engine);
                }
                KeyCode::Char('q') => {
                    engine.stop();
                    println!("Exiting...");
                    break Ok(());
                }
                _ => (),
            },
            _ => (),
        }
        // let mut buffer = String::new();
        // print!("> ");
        // io::stdout().flush()?;
        // io::stdin().read_line(&mut buffer)?;
        // match buffer.trim() {
        //     "quit" => {
        //         engine.stop();
        //         println!("Exiting...");
        //         break Ok(())
        //     },
        //     input => {
        //         // check if the name of a producer was typed
        //         match engine.get_producer(input) {
        //             Some(producer) => {
        //                 println!("Found {} producer", input);
        //                 let mut producer = producer.lock().unwrap();
        //                 let q = producer.production_quantity();
        //                 producer.set_production_quantity(q * 2.0);
        //                 println!("gold production per tick is {}", producer.production_quantity());
        //             },
        //             None => {
        //                 println!("No producer called {} found", input);
        //             }
        //         }
        //     },
        // }
    }
}

fn add_gold_mine(engine: &mut Engine) {
    if let Some(producer) = engine.get_producer("gold") {
        let mut producer = producer.lock().unwrap();
        let q = producer.production_quantity();
        producer.set_production_quantity(q * 2.0);
    } else {
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
    }
}

fn add_wood_cutter(engine: &mut Engine) {
    match engine.get_producer("wood") {
        Some(producer) => {
            let mut producer = producer.lock().unwrap();
            let q = producer.production_quantity();
            producer.set_production_quantity(q * 2.0);
        }
        None => {
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
    }
}

fn setup_producers(engine: &mut Engine) {
    // let producer = ProducerEntity {
    //     id: "gold".to_string(),
    //     base_cost: 1.0,
    //     cost_coefficient: 1.03,
    //     product_type: ProductType {
    //         name: "Gold".to_string(),
    //         production_quantity: 0.01,
    //         value_per_unit: 1.0,
    //     },
    //     production_time_ms: 1000,
    //     time_elapsed: 0,
    // };
    // engine.add_producer(Box::new(producer));
    // let producer = ProducerEntity {
    //     id: "wood".to_string(),
    //     base_cost: 1.0,
    //     cost_coefficient: 1.04,
    //     product_type: ProductType {
    //         name: "Wood".to_string(),
    //         production_quantity: 1.0,
    //         value_per_unit: 2.0,
    //     },
    //     production_time_ms: 500,
    //     time_elapsed: 0,
    // };
    // engine.add_producer(Box::new(producer));
}
