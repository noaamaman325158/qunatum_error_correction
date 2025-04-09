use crate::qubit::Qubit;
use num_complex::Complex64;

pub trait Gate {
    fn apply(&self, qubit: &mut Qubit);
}

pub struct PauliX;
pub struct PauliZ;

pub struct CNOT;

impl CNOT {
    pub fn apply(control: &mut Qubit, target: &mut Qubit){
        // Simple Implementation - if control is |1>, flip target
        if control.measure(){
            PauliX.apply(target);
        }
    }
}

impl Gate for PauliX {
    fn apply(&self, qubit: &mut Qubit){
        // Implement X Gate
        // Swap the amplitudes of |0⟩ and |1⟩
        //( bit flip)
        qubit.swap_state();
    }
}

impl Gate for PauliZ {
    fn apply(&self, qubit: &mut Qubit){
        // Implement Z Gate
        // Apply a phase flip to the |1> state
        // .re -> Real, .im -> Imaginary
        // The real part is negative of the original part
        // the imaginary part remains the same
        // Leave the |0> state unchanged but multiplies the |1> state by -1
        qubit.apply_phase_flip();
    }
}