use nalgebra::{Complex, DVector, Unit};
use crate::Qubit::Qubit;
use num_traits::One;
use num_traits::Zero;

pub(crate) struct QuantumRegister {
    pub state: Unit<DVector<Complex<f32>>>
}

impl QuantumRegister {
    pub fn new(qubit: Qubit) -> Self {
        Self{
            state: Unit::<DVector<Complex<f32>>>::new_normalize(DVector::from_vec(vec![qubit.state.x.into(), qubit.state.y.into()]))
        }
    }

    pub fn new_from_vec(vec: Vec<Complex<f32>>) -> Self {
        Self {state: Unit::<DVector<Complex<f32>>>::new_normalize(DVector::from_vec(vec))}
    }

    pub fn new_from_unit_vec(vec: Unit<DVector<Complex<f32>>>) -> Self{
        Self {state: vec}
    }

    /**
    Add new cubit using Kronecker productSince exactly 1 new qubit is being added, the result will be twice the size as before
    Each entry from the register must be combined with each entry in the cubit by multiplication
    This is very inefficient (which should be obvious since this is not running on a quantum computer)
    */
    pub fn add(&mut self, qubit: Qubit) {
        let mut new_vec: Vec<Complex<f32>> = vec![Complex::zero(); self.state.len() * 2];
        for i in 0..new_vec.len(){
            new_vec[i] = self.state[i/2] * qubit.state[i%2];
        }
        self.state = Unit::<DVector<Complex<f32>>>::new_normalize(DVector::from_vec(new_vec));
    }

    /**
    Instantiate a new empty register.
    Register must be filled before being used
    */
    pub fn new_empty(size: usize) -> Self {
        let vec: Vec<Complex<f32>> = vec![Complex::one(); size];
        Self {state: Unit::<DVector<Complex<f32>>>::new_normalize(DVector::from_vec(vec))}
    }

    pub fn change_state(&mut self, new_state: Vec<Complex<f32>>){
        self.state = Unit::<DVector<Complex<f32>>>::new_normalize(DVector::from_vec(new_state));
    }

    pub fn clone(&self) -> QuantumRegister {
        QuantumRegister::new_from_unit_vec(self.state.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let one: Complex<f32> = Complex::one();
        let qubit: Qubit = Qubit::new_zero_state();
        let register: QuantumRegister = QuantumRegister::new(qubit);
        // assert_eq!(one, register.state.data);
        for num in register.state.iter(){
            print!("num: {}\n", num.re)
        }
    }

    #[test]
    fn test_add() {
        let qubit: Qubit = Qubit::new_zero_state();
        let mut register: QuantumRegister = QuantumRegister::new(qubit);
        for i in 0..register.state.len(){
            print!("num: {}\n", register.state[i]);
        }
        print!("\n\n");
        let qubit: Qubit = Qubit::new_zero_state();
        // assert_eq!(one, register.state.data);
        register.add(qubit);
        for num in register.state.iter(){
            print!("num: {}\n", num.re)
        }
        print!("\n\n");
        let data: Vec<Complex<f32>> = vec![Complex::one(); 2];
        let qubit: Qubit = Qubit::new_from_vec(data);
        // assert_eq!(one, register.state.data);
        register.add(qubit);
        for num in register.state.iter(){
            print!("num: {}\n", num.re)
        }
    }
}