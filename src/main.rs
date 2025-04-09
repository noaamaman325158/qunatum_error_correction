// Imports for all code types
use crate::qubit::Qubit;
use crate::error_models::{BitFlipNoise, PhaseFlipNoise};
use crate::correction_codes::{BitFlipCode, PhaseFlipCode};
use crate::simulation::Simulation;
use crate::visualization::{plot_success_rates, plot_error_vs_success};

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

    // Generate basic comparison visualization
    println!("\n=== Generating Visualizations ===");
    match plot_success_rates(
        bit_flip_result.success_rate,
        phase_flip_result.success_rate,
        "success_rates.svg"
    ) {
        Ok(_) => println!("Success rates chart created successfully"),
        Err(e) => println!("Failed to create chart: {}", e),
    }

    // Run additional simulations with various error rates for trend analysis
    println!("\n=== Running Error Rate Comparison ===");
    let error_rates = vec![0.01, 0.05, 0.1, 0.15, 0.2, 0.25, 0.3];
    let mut bit_flip_success_rates = Vec::new();
    let mut phase_flip_success_rates = Vec::new();

    for error_rate in &error_rates {
        println!("Running simulations with error rate: {}", error_rate);

        // Run bit flip simulation with this error rate
        let bit_error_model = Box::new(BitFlipNoise::new(*error_rate));
        let bit_code = Box::new(BitFlipCode::new());
        let mut bit_sim = Simulation::new(bit_error_model, bit_code, 500);
        let bit_result = bit_sim.run();
        bit_flip_success_rates.push(bit_result.success_rate);

        // Run phase flip simulation with this error rate
        let phase_error_model = Box::new(PhaseFlipNoise::new(*error_rate));
        let phase_code = Box::new(PhaseFlipCode::new());
        let mut phase_sim = Simulation::new(phase_error_model, phase_code, 500);
        let phase_result = phase_sim.run();
        phase_flip_success_rates.push(phase_result.success_rate);
    }

    match plot_error_vs_success(
        &error_rates,
        &bit_flip_success_rates,
        &phase_flip_success_rates,
        "error_vs_success.svg"
    ) {
        Ok(_) => println!("Error vs success chart created successfully"),
        Err(e) => println!("Failed to create chart: {}", e),
    }

    println!("\nSimulations and visualizations complete!");
    println!("Results have been saved as 'success_rates.png' and 'error_vs_success.png'");
}