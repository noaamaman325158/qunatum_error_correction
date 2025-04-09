use crate::qubit::Qubit;
use crate::error_models::ErrorModel;
use crate::correction_codes::CorrectionCode;

pub struct Simulation {
    error_model: Box<dyn ErrorModel>,
    correction_code: Box<dyn CorrectionCode>,
    num_runs: usize
}

pub struct SimulationResult {
    pub success_rate: f64,
    pub error_rate: f64,
    pub average_correction_time: f64,
}

impl Simulation {
    pub fn new(
        error_model: Box<dyn ErrorModel>,
        correction_code: Box<dyn CorrectionCode>,
        num_runs: usize,
    ) -> Self {
        Self {
            error_model,
            correction_code,
            num_runs
        }
    }

    pub fn run(&self) -> SimulationResult{
        // Run the simulation multiple times
        let mut success_count = 0;

        for _ in 0..self.num_runs {
            // Create a qubit
            let mut qubit = Qubit::new();

            // Apply the error model
            self.error_model.apply_error(&mut qubit);

            // Encode the qubit using the correction code
            let mut encoded_qubits = self.correction_code.encode(&qubit);

            // Measure the syndromes
            let syndromes = self.correction_code.syndrome_measurement(&mut encoded_qubits);

            // Correct the errors
            self.correction_code.correct(&mut encoded_qubits, syndromes);

            // Decode the qubit
            let decoded_qubit = self.correction_code.decode(&mut encoded_qubits);

            // Check if the decoded qubit is in the correct state
            if decoded_qubit == qubit {
                success_count += 1;
            }
        }

        // Collect the statistics on error correction performance
        let success_rate = success_count as f64 / self.num_runs as f64;
        let error_rate = 1.0 - success_rate;
        let average_correction_time = self.correction_code.get_average_correction_time();

        SimulationResult {
            success_rate,
            error_rate,
            average_correction_time,
        }
    }
}