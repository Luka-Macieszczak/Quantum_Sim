use nalgebra::{Complex, Vector2, UnitVector2, Normed};
use num_traits::identities::One;
use num_traits::Zero;
use crate::Quaternion::Quaternion;
use rand::Rng;

pub struct Qubit {
    pub state: UnitVector2<Complex<f32>>
}

impl Qubit {

    pub fn new_zero_state() -> Self {
        Self {state: UnitVector2::new_normalize(Vector2::new(Complex::one(), Complex::zero()))}
    }

    pub fn new_one_state() -> Self {
        Self {state: UnitVector2::new_normalize(Vector2::new(Complex::zero(), Complex::one()))}
    }

    pub fn new_from_vec(vec: Vec<Complex<f32>>) -> Self {
        Self {state: UnitVector2::new_normalize(Vector2::from_vec(vec))}
    }

    /*
    pub fn new_from_quaternion(quaternion: Quaternion) -> Self{
        Self {state: UnitVector2::new_normalize(Vector2::new(Complex::one() * quaternion.a + Complex::i() * quaternion.b,
                                                             Complex::one() * quaternion.c + Complex::i() * quaternion.d))}
    }
    */


    /**
    Return a list of qubits all in classical states corresponding to the j'th vector in dirac notation (imagine that j is in binary)
    |0> => (1, 0), |00> => (1,0,0,0), |00> = |0>⊗|0>, |1> = (0,1)
    |j> = tensor product of all of it's binary digits
    */
    pub fn get_qubits_from_state(mut j: i32, mut num_qubits: i32) -> Vec<Qubit> {
        let mut vec: Vec<Qubit> = vec![];
        let total_qubits: i32 = num_qubits.clone();
        while j > 0 && vec.len() < total_qubits as usize {
            let cur = j & 1;
            if cur == 1{
                vec.push(Qubit::new_one_state());
            }
            else if cur == 0 {
                vec.push(Qubit::new_zero_state());
            }
            j = j>>1;
            num_qubits -= 1;
        }
        for _ in 0..num_qubits{
            vec.push(Qubit::new_zero_state());
        }
        vec
    }

    /**
    Measure individual qubit and collapse quantum state
    return a zero or 1 corresponding to it's classical state
    */

    pub fn measure(&mut self) -> i32{
        let rng = rand::thread_rng().gen_range(0. .. 1.);
        return if rng <= self.state.x.norm() {
            self.state = UnitVector2::new_normalize(Vector2::new(Complex::one(), Complex::zero()));
            0
        } else {
            self.state = UnitVector2::new_normalize(Vector2::new(Complex::zero(), Complex::one()));
            1
        }
        // Something goofy happened if you get here
        -1
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let one : Complex<f32> = Complex::one();
        let qubit : Qubit = Qubit::new_zero_state();
        assert_eq!(one, qubit.state.x.into());
    }

    #[test]
    fn test_init_from_quaternion() {
        let one: Complex<f32> = Complex::one();
        let i : Complex<f32> = Complex::i();
        let quaternion: Quaternion = Quaternion::new_from_coefficients(3., 8., 9., 10.);
        // let qubit: Qubit = Qubit::new_from_quaternion(quaternion);

        let mut quaternion2: Quaternion = Quaternion::new_from_coefficients(3., 8., 9., 10.);
        quaternion2.normalize();

        // assert_eq!(quaternion2.a * one + quaternion2.b * i, qubit.state.x.into())
    }

    #[test]
    fn get_from_state_test(){
        let vec: Vec<Qubit> = Qubit::get_qubits_from_state(5, 3);
        for i in 0..vec.len(){
            print!("State: ({}, {})\n", vec[i].state.x, vec[i].state.y);
        }
    }
}
