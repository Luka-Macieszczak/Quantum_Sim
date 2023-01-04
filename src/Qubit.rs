use nalgebra::{Complex, Vector2, UnitVector2, Unit, Vector4};
use num_traits::identities::One;
use num_traits::Zero;
use crate::Quaternion::Quaternion;

pub(crate) struct Qubit {
    pub(crate) state: UnitVector2<Complex<f32>>
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


    pub fn new_from_quaternion(quaternion: Quaternion) -> Self{
        Self {state: UnitVector2::new_normalize(Vector2::new(Complex::one() * quaternion.a + Complex::i() * quaternion.b,
                                                             Complex::one() * quaternion.c + Complex::i() * quaternion.d))}
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
        let qubit: Qubit = Qubit::new_from_quaternion(quaternion);

        let mut quaternion2: Quaternion = Quaternion::new_from_coefficients(3., 8., 9., 10.);
        quaternion2.normalize();

        assert_eq!(quaternion2.a * one + quaternion2.b * i, qubit.state.x.into())
    }
}
