use create::qubit::Qubit;
use num_complex::Complex64;

pub trait Gate {
    fn apply(&self, qubit: &mut Qubit);
}

pub struct PauliX;

impl Gate for PauliX {
    fn apply(&self, qubit: &mut Qubit){
        // Implement X Gate
        // Swap the amplitudes of |0⟩ and |1⟩
        //( bit flip)
        qubit.state.swap(0, 1);
    }
}