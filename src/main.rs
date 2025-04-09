// Fix imports to use crate:: instead of create::
use crate::qubit::Qubit;
use crate::error_models::BitFlipNoise;
use crate::correction_codes::BitFlipCode;
use crate::simulation::Simulation;

mod qubit;
mod gates;
mod error_models;
mod correction_codes;
mod simulation;
mod visualization;

fn main() {
    println!("Quantum Error Correction Simulator");

    // Create an error model with 10% error probability
    // Fix: Use BitFlipNoise instead of BitFlipCode
    let error_model = Box::new(BitFlipNoise::new(0.1));

    // Create a bit flip correction code
    let correction_code = Box::new(BitFlipCode::new());

    // Set up simulation with 1000 runs
    let mut simulation = Simulation::new(error_model, correction_code, 1000);


    // Run the simulation
    let result = simulation.run();

    println!("Simulation Results:");
    println!("Success Rate: {:.2}%", result.success_rate * 100.0);
    println!("Error Rate: {:.2}%", result.error_rate * 100.0);
    println!("Average Correction Time: {:.2} seconds", result.average_correction_time);
}