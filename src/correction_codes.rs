use crate::qubit::Qubit;
use crate::gates::*;
use std::time::{Duration, Instant};

pub trait CorrectionCode {
    fn encode(&self, data: &Qubit) -> Vec<Qubit>;
    fn syndrome_measurement(&self, encoded_qubits: &mut Vec<Qubit>) -> Vec<bool>;
    fn correct(&mut self, encoded_qubits: &mut Vec<Qubit>, syndromes: Vec<bool>); // Changed to &mut self
    fn decode(&self, encoded_qubits: &mut Vec<Qubit>) -> Qubit;

    fn get_average_correction_time(&self) -> f64;
}

pub struct BitFlipCode{
    correction_time: Vec<Duration>,
}

impl BitFlipCode{
    pub fn new() -> Self {
        Self {
            correction_time: Vec::new(),
        }
    }
}

// Only declare PhaseFlipCode once
pub struct PhaseFlipCode {
    correction_time: Vec<Duration>,
}

impl PhaseFlipCode {
    pub fn new() -> Self {
        Self {
            correction_time: Vec::new(),
        }
    }
}

impl CorrectionCode for BitFlipCode {
    /// Implement the Bit Flip
    fn encode(&self, data: &Qubit) -> Vec<Qubit> {
        // Create three qubits with the same state as the input qubit
        let mut encoded_qubits = vec![data.clone(), data.clone(), data.clone()];

        // Split the vector into disjoint mutable slices
        let (q0_slice, rest) = encoded_qubits.split_at_mut(1);
        let q0 = &mut q0_slice[0];
        let (q1, q2) = rest.split_at_mut(1);

        // Apply CNOT gates
        CNOT::apply(q0, &mut q1[0]);
        CNOT::apply(q0, &mut q2[0]);

        // Return the encoded qubits
        encoded_qubits
    }

    fn syndrome_measurement(&self, encoded_qubits: &mut Vec<Qubit>) -> Vec<bool> {
        // For a 3-qubit bit flip code, we need 2 syndrome bits
        let mut syndromes = vec![false, false];

        // Get measurement results (we should measure only once per qubit)
        let q0_result = encoded_qubits[0].measure();
        let q1_result = encoded_qubits[1].measure();
        let q2_result = encoded_qubits[2].measure();

        // Calculate syndromes
        // First syndrome is parity of qubit 0 and 1
        syndromes[0] = q0_result ^ q1_result;

        // Second syndrome is parity of qubit 0 and 2
        syndromes[1] = q0_result ^ q2_result;

        syndromes
    }

    fn correct(&mut self, encoded_qubits: &mut Vec<Qubit>, syndromes: Vec<bool>) {
        let start = Instant::now();

        // Use the syndromes parameter instead of looking for a syndromes variable
        if syndromes.len() >= 2 {
            if syndromes[0] && syndromes[1] {
                PauliX.apply(&mut encoded_qubits[0]);
            }
            // Only first syndrome true: error on qubit 1
            else if syndromes[0] {
                PauliX.apply(&mut encoded_qubits[1]);
            }
            // Only second syndrome true: error on qubit 2
            else if syndromes[1] {
                PauliX.apply(&mut encoded_qubits[2]);
            }
        }

        let duration = start.elapsed();

        self.correction_time.push(duration);
    }

    fn decode(&self, encoded_qubits: &mut Vec<Qubit>) -> Qubit {
        // Return the corrected logical qubit
        let mut decoded_qubit = encoded_qubits[0].clone();

        // Apply CNOT gates to decode the qubits
        CNOT::apply(&mut decoded_qubit, &mut encoded_qubits[1]);
        CNOT::apply(&mut decoded_qubit, &mut encoded_qubits[2]);

        // Return the decoded qubit
        decoded_qubit
    }

    fn get_average_correction_time(&self) -> f64 {
        if self.correction_time.is_empty(){
            return 0.0
        }

        let total = self.correction_time.iter()
            .fold(Duration::new(0, 0), |acc, &x| acc + x);

        total.as_secs_f64() / self.correction_time.len() as f64
    }
}

// Keep only ONE implementation for PhaseFlipCode
impl CorrectionCode for PhaseFlipCode {
    fn encode(&self, data: &Qubit) -> Vec<Qubit> {
        // Create three qubits in the |+⟩ state if data is |+⟩, or |-⟩ state if data is |-⟩
        let mut encoded_qubits = vec![data.clone(), data.clone(), data.clone()];

        // Apply Hadamard gates to convert to the |+⟩/|-⟩ basis
        for qubit in &mut encoded_qubits {
            Hadamard.apply(qubit);
        }

        // Apply CZ gates (controlled-Z) from first qubit to others
        // Since you might not have CZ implemented, we can simulate with H + CNOT + H
        let (q0_slice, rest) = encoded_qubits.split_at_mut(1);
        let q0 = &mut q0_slice[0];
        let (q1, q2) = rest.split_at_mut(1);

        // Apply H to target, CNOT, then H again to simulate CZ
        Hadamard.apply(&mut q1[0]);
        CNOT::apply(q0, &mut q1[0]);
        Hadamard.apply(&mut q1[0]);

        Hadamard.apply(&mut q2[0]);
        CNOT::apply(q0, &mut q2[0]);
        Hadamard.apply(&mut q2[0]);

        encoded_qubits
    }

    fn syndrome_measurement(&self, encoded_qubits: &mut Vec<Qubit>) -> Vec<bool> {
        // Convert to X-basis for measurement
        for qubit in encoded_qubits.iter_mut() {
            Hadamard.apply(qubit);
        }

        // Now measure in computational basis
        let q0_result = encoded_qubits[0].measure();
        let q1_result = encoded_qubits[1].measure();
        let q2_result = encoded_qubits[2].measure();

        // Convert back to Z-basis
        for qubit in encoded_qubits.iter_mut() {
            Hadamard.apply(qubit);
        }

        // Calculate parity checks
        let syndromes = vec![
            q0_result ^ q1_result,  // Parity between qubit 0 and 1
            q0_result ^ q2_result   // Parity between qubit 0 and 2
        ];

        syndromes
    }

    fn correct(&mut self, encoded_qubits: &mut Vec<Qubit>, syndromes: Vec<bool>) {
        let start = Instant::now();

        if syndromes.len() >= 2 {
            // Both syndromes true: error on qubit 0
            if syndromes[0] && syndromes[1] {
                PauliZ.apply(&mut encoded_qubits[0]);
            }
            // Only first syndrome true: error on qubit 1
            else if syndromes[0] {
                PauliZ.apply(&mut encoded_qubits[1]);
            }
            // Only second syndrome true: error on qubit 2
            else if syndromes[1] {
                PauliZ.apply(&mut encoded_qubits[2]);
            }
        }

        let duration = start.elapsed();
        self.correction_time.push(duration);
    }

    fn decode(&self, encoded_qubits: &mut Vec<Qubit>) -> Qubit {
        // Apply Hadamard gates to convert back to computational basis
        for qubit in encoded_qubits.iter_mut() {
            Hadamard.apply(qubit);
        }

        // Extract the logical qubit (similar to bit flip code)
        let mut decoded_qubit = encoded_qubits[0].clone();

        // Apply CNOT gates to decode
        CNOT::apply(&mut decoded_qubit, &mut encoded_qubits[1]);
        CNOT::apply(&mut decoded_qubit, &mut encoded_qubits[2]);

        // Convert back from |+⟩/|-⟩ basis to computational basis
        Hadamard.apply(&mut decoded_qubit);

        decoded_qubit
    }

    fn get_average_correction_time(&self) -> f64 {
        if self.correction_time.is_empty(){
            return 0.0
        }

        let total = self.correction_time.iter()
            .fold(Duration::new(0, 0), |acc, &x| acc + x);

        total.as_secs_f64() / self.correction_time.len() as f64
    }
}