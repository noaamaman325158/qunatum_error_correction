use crate::qubit::Qubit;
use num_complex::Complex64;

pub trait Gate {
    fn apply(&self, qubit: &mut Qubit);
}



pub struct CNOT;

impl CNOT {
    pub fn apply(control: &mut Qubit, target: &mut Qubit){
        // Simple Implementation - if control is |1>, flip target
        if control.measure(){
            PauliX.apply(target);
        }
    }
}

pub struct PauliX;
impl Gate for PauliX {
    fn apply(&self, qubit: &mut Qubit) {
        // Use the accessor method we defined
        qubit.swap_state();
    }
}

pub struct PauliZ;
impl Gate for PauliZ {
    fn apply(&self, qubit: &mut Qubit) {
        // Use the accessor method we defined
        qubit.apply_phase_flip();
    }
}

pub struct Hadamard;
impl Gate for Hadamard {
    fn apply(&self, qubit: &mut Qubit) {
        // H|0⟩ = |+⟩ = (|0⟩ + |1⟩)/√2
        // H|1⟩ = |-⟩ = (|0⟩ - |1⟩)/√2
        let old_state = qubit.get_state();

        // Calculate new amplitudes
        let normalization = Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0);
        qubit.set_state(0, normalization * (old_state[0] + old_state[1]));
        qubit.set_state(1, normalization * (old_state[0] - old_state[1]));
    }
}