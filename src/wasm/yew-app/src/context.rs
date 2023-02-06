use nalgebra::{Complex, Vector2, UnitVector2};
use num_traits::One;
use num_traits::Zero;
pub fn create_state(input: i32) -> Vec<Vec<UnitVector2<Complex<f32>>>>{
    let num_qubits: i32 = ((input as f32).log2().ceil()) as i32;
    let mut data: Vec<Vec<UnitVector2<Complex<f32>>>> = vec![vec![]];
    for i in 0..num_qubits{
        data[0].push(UnitVector2::new_normalize(Vector2::new(Complex::zero(), Complex::one())));
    }
    data
}