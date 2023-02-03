pub mod Qubit;
pub mod Quaternion;
pub mod Gates;
pub mod Matrix;
pub mod QuantumRegister;
pub mod QuantumCircuit;
pub mod Euclid;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
