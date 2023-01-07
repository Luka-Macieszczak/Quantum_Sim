use nalgebra::{Complex, DVector, Normed, Unit};
use crate::Qubit::Qubit;
use num_traits::One;
use num_traits::Zero;
use rand::Rng;

pub(crate) struct QuantumRegister {
    pub state: Unit<DVector<Complex<f32>>>
}

impl QuantumRegister {
    pub fn new(qubit: Qubit) -> Self {
        Self{
            state: Unit::<DVector<Complex<f32>>>::new_normalize(DVector::from_vec(vec![qubit.state.x.into(), qubit.state.y.into()]))
        }
    }

    pub fn new_from_int(index: usize, size: usize) -> Self {
        let mut state: Vec<Complex<f32>> = vec![Complex::zero() ; size];
        state[index] = Complex::one();
        Self {
            state: Unit::<DVector<Complex<f32>>>::new_normalize(DVector::from_vec(state))
        }
    }

    pub fn new_from_vec(vec: Vec<Complex<f32>>) -> Self {
        Self {state: Unit::<DVector<Complex<f32>>>::new_normalize(DVector::from_vec(vec))}
    }

    pub fn new_from_unit_vec(vec: Unit<DVector<Complex<f32>>>) -> Self{
        Self {state: vec}
    }

    /**
    Add new cubit using Kronecker product since exactly 1 new qubit is being added, the result will be twice the size as before
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
    Measure the quantum register and collapse it's state onto the outcome of the measurement
    Return an integer corresponding to the state of the qubits in the classical state
    */
    pub fn measure(&mut self) -> i32 {
        let rng = rand::thread_rng().gen_range(0. .. 1.);
        let mut current_probability = 0.;
        let mut new_state: Vec<Complex<f32>> = vec![];
        for i in 0..self.state.len(){
            let norm: f32 = self.state[i].norm();
            current_probability += norm;
            if current_probability >= rng{
                new_state.push(Complex::one());
                new_state.append(&mut vec![Complex::zero() ; self.state.len() - i - 1]);
                self.change_state(new_state);
                return i as i32
            } else {
                new_state.push(Complex::zero());
            }
        }
        -1
    }
    /**
    Peak at a possible measurement value of the state of the qubits but don't collapse the state
    */
    pub fn peak(&self) -> i32 {
        let rng = rand::thread_rng().gen_range(0. .. 1.);
        let mut current_probability = 0.;
        for i in 0..self.state.len(){
            let norm: f32 = self.state[i].norm();
            current_probability += norm;
            if current_probability >= rng{
                return i as i32
            }
        }
        -1
    }

    /**
    Gets the state of each individual qubit by adding the norm of each element in the register that corresponds to each qubit
    Each element in a register is a product of some qubit state vector elements (i.e. (x1y1, x1y2, x2y1, x2y2))
    Every x1 is in the 1st half, and every x2 is in the 2nd half. Every y1 is in the first and 3rd quarter, every y2 is in the 2nd and 4th quarter
    The position of each state element alternates every k/(2^i) where k is the length of the register and i is the ith qubit added
    After a gate is applied, the state is a linear combination of all of the elements of the register, which are all products of the original qubit state elements
    Factoring out each qubit state element and taking the sum of the norms of its positions in the register following the alternating rule will give the qubit state element after the gate has been applied
    */
    pub fn get_qubit_norms(&self) -> Vec<Qubit> {
        let mut vec: Vec<Qubit> = vec![];
        for i in 0..(self.state.len() as f32).log2() as i32{
            let mut x1: Complex<f32> = Complex::zero();
            let mut x2: Complex<f32> = Complex::zero();
            for j in 0..self.state.len(){
                // Need to access pow method
                let d: i32 = 2;
                let divider: i32 = (self.state.len() as i32 / d.pow(i as u32 + 1));

                print!("Condition: {}\n", j as i32 / divider);

                if (j as i32 / divider) % 2 == 0{
                    x1 += self.state[j].norm();
                }
                else {
                    x2 += self.state[j].norm();
                }
            }
            vec.push(Qubit::new_from_vec(vec![x1, x2]))
        }
        vec
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