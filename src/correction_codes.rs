use crate::qubit::Qubit;
use crate::gates::*;

pub trait CorrectionCode{
    fn encode(&self, data: &Qubit) -> Vec<Qubit>;
    fn syndrome_measurement(&self, encoded_qubits: &mut Vec<Qubit>) -> Vec<bool>;
    fn correct(&self, encoded_qubits: &mut Vec<Qubit>, syndromes: Vec<bool>);
    fn decode(&self, encoded_qubits: &mut Vec<Qubit>) -> Qubit;

    fn get_average_correction_time(&self) -> f64;
}

pub struct BitFlipCode;

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

    fn correct(&self, encoded_qubits: &mut Vec<Qubit>, syndromes: Vec<bool>) {
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
        // Placeholder implementation, replace with actual logic if needed
        0.01
    }
}

