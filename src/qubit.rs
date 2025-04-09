use num_complex::Complex64;


#[derive(Clone)]
pub struct Qubit {
    /// The state of the qubit, represented as a 2D array of complex numbers.
    /// The first element represents the amplitude of |0⟩, and the second element represents the amplitude of |1⟩.
    state: [Complex64; 2],
}

impl Qubit {
    pub fn new() -> Self {
        /// Initialize to |0> state
        Self {
            state: [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)],
        }
    }

    pub fn measure(&self) -> bool {
        let zero_prob = self.state[0].norm_sqr();
        let random = rand::random::<f64>();
        random > zero_prob
    }

    pub fn swap_state(&mut self){
        self.state.swap(0, 1);
    }

    pub fn apply_phase_flip(&mut self){
        self.state[1] = Complex64::new(-self.state[1].re, self.state[1].im);
    }


}

impl PartialEq for Qubit {
    fn eq(&self, other: &Self) -> bool {
        self.state[0] == other.state[0] && self.state[1] == other.state[1]
    }
}