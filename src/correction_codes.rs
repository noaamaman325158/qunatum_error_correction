use create::qubit::Qubit;
use create::gates::*;

pub trait CorrectionCode{
    fn encode(&self, data: &Qubit) -> Vec<Qubit>;
    fn syndrome_measurement(&self, encoded_qubits: &mut Vec<Qubit>) -> Vec<bool>;
    fn correct(&self, encoded_qubits: &mut Vec<Qubit>, syndromes: Vec<bool>);
    fn decode(&self, encoded_qubits: &mut Vec<Qubit>) -> Qubit;
}

pub struct BitFlipCode;

impl CorrectionCode for BitFlipCode {
    /// Implement the Bit Flip
    fn encode(&self, data: &Qubit) -> Vec<Qubit>{
        // Create three qubits with the same state as the input qubit
        let mut encoded_qubits = vec![data.clone(), data.clone(), data.clone()];

        // Apply a CNOT gate to the first two qubits
        CNOT.apply(&mut encoded_qubits[0], &mut encoded_qubits[1]);
        CNOT.apply(&mut encoded_qubits[0], &mut encoded_qubits[2]);

        // Return the encoded qubits
        encoded_qubits
    }

    fn syndrome_measurement(&self, encoded_qubits: &mut Vec<Qubit>) -> Vec<bool> {
        // Measure syndromes (parity Checks)
        let mut syndromes = vec![false, false];
        // Measure the first two qubits
        let first_syndrome = encoded_qubits[0].measure() ^ encoded_qubits[1].measure();
        let second_syndrome = encoded_qubits[0].measure() ^ encoded_qubits[2].measure();

        // Store the results in the syndromes vector
        syndromes[0] = first_syndrome;
        syndromes[1] = second_syndrome;

        // Return the syndromes
        syndromes
    }

    fn correct(&self, encoded_qubits: &Vec<Qubit>) -> Qubit {
        // Apply correction based on the syndrome
        let mut corrected_qubits = encoded_qubits.clone();

        // Check the syndromes and apply corrections
        if syndromes[0] {
            // Apply PauliX to the first qubit
            PauliX.apply(&mut corrected_qubits[0]);
        }

        if syndromes[1] {
            // Apply PauliX to the second qubit
            PauliX.apply(&mut corrected_qubits[1]);
        }

        if syndromes[2] {
            // Apply PauliX to the third qubit
            PauliX.apply(&mut corrected_qubits[2]);
        }

        // Return the corrected qubit
        corrected_qubits[0].clone()
    }

    fn decode(&self, encoded_qubits: &Vec<Qubit>) -> Qubit {
        // Return the corrected logical qubit
        let mut decoded_qubit = encoded_qubits[0].clone();

        // Apply CNOT gates to decode the qubits
        CNOT.apply(&mut decoded_qubit, &mut encoded_qubits[1]);
        CNOT.apply(&mut decoded_qubit, &mut encoded_qubits[2]);

        // Return the decoded qubit
        decoded_qubit
    }
}

