use std::fmt::Debug;

pub trait ProductType: Debug + Send {
    fn get_name(&self) -> &str;
}

pub trait Producer: Debug + Send {
    fn on_tick(&self, delta: u128);
}

#[derive(Debug, Copy, Clone)]
pub struct ProducerEntity<T: ProductType> {
    pub base_cost: f64,
    pub cost_coefficient: f64,
    pub output: T,
    pub production_time_ms: u64,
}

impl<T: ProductType + Debug> ProducerEntity<T> {
    pub fn new(product: T) -> Self {
        ProducerEntity {
            base_cost: 0.0,
            cost_coefficient: 0.0,
            output: product,
            production_time_ms: 0,
        }
    }

}

impl<T: ProductType + Send + Sync + Debug> Producer for ProducerEntity<T> {
    fn on_tick(&self, delta: u128) {
        println!("Product.on_tick({}) for {:?}", delta, self.output);
    }
}