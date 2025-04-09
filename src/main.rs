// Imports for all code types
use crate::qubit::Qubit;
use crate::error_models::{BitFlipNoise, PhaseFlipNoise};
use crate::correction_codes::{BitFlipCode, PhaseFlipCode};
use crate::simulation::Simulation;

mod qubit;
mod gates;
mod error_models;
mod correction_codes;
mod simulation;
mod visualization;

fn main() {
    println!("Quantum Error Correction Simulator");

    // Run bit flip code simulation
    println!("\n=== Bit Flip Code Simulation ===");

    // Create an error model with 10% error probability
    let bit_flip_error_model = Box::new(BitFlipNoise::new(0.1));

    // Create a bit flip correction code
    let bit_flip_code = Box::new(BitFlipCode::new());

    // Set up simulation with 1000 runs
    let mut bit_flip_simulation = Simulation::new(bit_flip_error_model, bit_flip_code, 1000);

    // Run the simulation
    let bit_flip_result = bit_flip_simulation.run();

    println!("Simulation Results:");
    println!("Success Rate: {:.2}%", bit_flip_result.success_rate * 100.0);
    println!("Error Rate: {:.2}%", bit_flip_result.error_rate * 100.0);
    println!("Average Correction Time: {:.2} seconds", bit_flip_result.average_correction_time);

    // Run phase flip code simulation
    println!("\n=== Phase Flip Code Simulation ===");

    // Create a phase flip error model with 10% error probability
    let phase_flip_error_model = Box::new(PhaseFlipNoise::new(0.1));

    // Create a phase flip correction code
    let phase_flip_code = Box::new(PhaseFlipCode::new());

    // Set up simulation with 1000 runs
    let mut phase_flip_simulation = Simulation::new(phase_flip_error_model, phase_flip_code, 1000);

    // Run the simulation
    let phase_flip_result = phase_flip_simulation.run();

    println!("Simulation Results:");
    println!("Success Rate: {:.2}%", phase_flip_result.success_rate * 100.0);
    println!("Error Rate: {:.2}%", phase_flip_result.error_rate * 100.0);
    println!("Average Correction Time: {:.2} seconds", phase_flip_result.average_correction_time);
}