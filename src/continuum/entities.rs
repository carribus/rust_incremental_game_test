use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ProductType {
    pub name: String,
    pub production_quantity: f64,
    pub value_per_unit: f64,
}

impl ProductType {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn production_quantity(&self) -> f64 {
        self.production_quantity
    }

    pub fn set_production_quantity(&mut self, quantity: f64) {
        self.production_quantity = quantity;
    }

    pub fn value_per_unit(&self) -> f64 {
        self.value_per_unit
    }
}


/// Any producer in the system must implement the `Producer` trait
pub trait Producer: Debug + Send {
    fn product_type(&self) -> &ProductType;
    fn on_tick(&mut self, delta: u64) -> f64;
}

#[derive(Debug, Clone)]
pub struct ProducerEntity {
    pub id: String,
    pub base_cost: f64,
    pub cost_coefficient: f64,
    pub product_type: ProductType,
    pub production_time_ms: u64,
    pub time_elapsed: u64,
}

impl Producer for ProducerEntity {
    fn product_type(&self) -> &ProductType {
        &self.product_type
    }

    fn on_tick(&mut self, delta: u64) -> f64 {
        self.time_elapsed += delta;

        let quantity_produced = {
            if self.time_elapsed > self.production_time_ms {
                let quantity_produced = (self.time_elapsed as f64 / self.production_time_ms as f64).floor() * self.product_type.production_quantity();
    
                self.time_elapsed -= self.production_time_ms * (quantity_produced / self.product_type.production_quantity()) as u64;
                quantity_produced
            } else {
                0.0
            }
        };

        quantity_produced
    }
}