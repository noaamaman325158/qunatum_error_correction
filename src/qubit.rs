use num_complex::Complex64;

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


}