use std::fmt::Debug;

pub trait ProductType: Debug + Send {
    fn name(&self) -> &str;
    fn production_quantity(&self) -> f64;
    fn set_production_quantity(&mut self, quantity: f64);
}

pub trait Producer: Debug + Send {
    fn product(&self) -> &dyn ProductType;
    fn on_tick(&mut self, delta: u64) -> f64;
}

#[derive(Debug, Copy, Clone)]
pub struct ProducerEntity<T: ProductType> {
    pub base_cost: f64,
    pub cost_coefficient: f64,
    pub output: T,
    pub production_time_ms: u64,
    pub time_elapsed: u64,
}

impl<T: ProductType + Debug> ProducerEntity<T> {
}

impl<T: ProductType + Send + Sync + Debug> Producer for ProducerEntity<T> {
    fn product(&self) -> &dyn ProductType {
        &self.output
    }

    fn on_tick(&mut self, delta: u64) -> f64 {
        // println!("Product.on_tick({}) for {:?}", delta, self.output);

        self.time_elapsed += delta;

        let quantity_produced = {
            if self.time_elapsed > self.production_time_ms {
                let quantity_produced = (self.time_elapsed as f64 / self.production_time_ms as f64).floor();
    
                self.time_elapsed -= ((self.production_time_ms as f64) * quantity_produced) as u64;
                quantity_produced
            } else {
                0.0
            }
        };

        // println!("{} {} produced", quantity_produced, self.output.name());

        quantity_produced
    }
}