use crate::qubit::Qubit;
use crate::gates::{PauliX, PauliZ, Gate};

pub trait ErrorModel {
    fn apply_error(&self, qubit: &mut Qubit);
}


pub struct BitFlipNoise {
    probability: f64,
}


impl BitFlipNoise{
    pub fn new(probability: f64) -> Self {
        Self { probability }
    }
}

impl ErrorModel for BitFlipNoise {
    fn apply_error(&self, qubit: &mut Qubit){
        if rand::random::<f64>() < self.probability {
            PauliX.apply(qubit);
        }
    }
}

