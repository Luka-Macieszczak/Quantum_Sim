use std::alloc::System;
use std::error::Error;
use std::io;
use nalgebra::{Complex, ComplexField, RealField};
use num_traits::One;
use crate::Matrix::Matrix;
use crate::QuantumRegister::QuantumRegister;
use crate::Qubit::Qubit;
use num_traits::Zero;

pub(crate) struct Gate {
    matrix: Matrix
}

impl Gate {
    // Pauli I gate (or classical identity)
    pub fn new_identity() -> Self {
        let mut matrix: Matrix = Matrix::new_identity(2);
        Self {matrix}
    }

    // Pauli X gate (or classical NOT gate)
    pub fn new_not() -> Self {
        let mut matrix: Matrix = Matrix::new_zero(2);
        matrix.rows[0][1] = Complex::one();
        matrix.rows[1][0] = Complex::one();
        Self {matrix}
    }

    // Pauli Y gate
    pub fn new_y() -> Self {
        let mut matrix: Matrix = Matrix::new_zero(2);
        matrix.rows[0][1] = Complex::i() * (-1.);
        matrix.rows[1][0] = Complex::i();
        Self {matrix}
    }
    // Pauli Z gate
    pub fn new_z() -> Self {
        let mut matrix: Matrix = Matrix::new_zero(2);
        matrix.rows[0][0] = Complex::one();
        matrix.rows[1][1] = Complex::one() * (-1.);
        Self {matrix}
    }

    // Hadamard gate
    pub fn new_h() -> Self {
        let matrix: Matrix = Matrix::new_h();
        Self {matrix}
    }
    // pi/8 gate. Called pi/8 because of a different written form
    pub fn new_t() -> Self{
        let num: Complex<f32> = (Complex::one() * f32::e()).powc(Complex::i()/(4.)*f32::pi());
        let mut matrix: Matrix = Matrix::new(vec![vec![Complex::one(), Complex::zero()], vec![Complex::zero(), num]]);
        Self {matrix}
    }

    // Controlled not gate. Reversible version of the not gate.
    // All n-qubit gates can be created from the hadamard gate, pi/8 gate and cnot gate (using infinitely many combinations)
    pub fn new_cnot() -> Self{
        let matrix: Matrix = Matrix::new(vec![vec![Complex::one(), Complex::zero(), Complex::zero(), Complex::zero()
        ], vec![Complex::zero(), Complex::one(), Complex::zero(), Complex::zero()], vec![Complex::zero(), Complex::zero(), Complex::zero(), Complex::one()],
        vec![Complex::zero(), Complex::zero(), Complex::one(), Complex::zero()]]);

        Self {matrix}
    }

    /**
    Combine 2 tensor products to get the cnot gate
    Each tensor product represents what to do in each state (nothing if the control bit is 0, flip if control is 1)
    Gate is obtained by tensoring I n - 2 many times (where n is the number of qubits), then choosing 2 spots designated by the control bit index and target bit index
    (Ex let control_qubit = 0, target_qubit = 2, num_qubits = 3: |0><0|⊗I⊗I + |1><1|⊗I⊗X. I is the identity matrix, X is the not matrix, |0> = (1,0), |1> = (0,1))
    This must be done twice for the zero state and the one state of the control bit
    Inputs are zero indexed
    FIX
    */
    pub fn new_multi_cnot(control_qubit: i32, target_qubit: i32, num_qubits: i32) -> Self{
        let identity: Matrix = Matrix::new_identity(2);
        // Outer product of a classical 0 bit in vector representation
        let zero_state_outer_product = Matrix::new(vec![vec![Complex::one(), Complex::zero()], vec![Complex::zero(), Complex::zero()]]);
        // Outer product of a classical 1 bit in vector representation
        let one_state_outer_product = Matrix::new(vec![vec![Complex::zero(), Complex::zero()], vec![Complex::zero(), Complex::one()]]);
        // Matrix of the not gate
        let not_matrix: Matrix = Matrix::new(vec![vec![Complex::zero(), Complex::one()], vec![Complex::one(), Complex::zero()]]);

        let mut zero_condition_matrix: Matrix = Matrix::new_zero(2);
        let mut one_condition_matrix: Matrix = Matrix::new_zero(2);

        if control_qubit == 0 {
            zero_condition_matrix.rows[0][0] = Complex::one();
            one_condition_matrix.rows[1][1] = Complex::one();
        }
        else if target_qubit == 0 {
            zero_condition_matrix.rows[1][0] = Complex::one();
            zero_condition_matrix.rows[0][1] = Complex::one();
            one_condition_matrix.rows[1][0] = Complex::one();
            one_condition_matrix.rows[0][1] = Complex::one();
        }
        else {
            zero_condition_matrix.rows[0][0] = Complex::one();
            zero_condition_matrix.rows[1][1] = Complex::one();
            one_condition_matrix.rows[0][0] = Complex::one();
            one_condition_matrix.rows[1][1] = Complex::one();
        }

        // Apply chain of tensors to each matrix
        for i in 1..num_qubits {
            if i == control_qubit{
                zero_condition_matrix = zero_condition_matrix.tensor_product(&zero_state_outer_product);
                one_condition_matrix = one_condition_matrix.tensor_product(&one_state_outer_product);
            }
            else if i == target_qubit {
                zero_condition_matrix = zero_condition_matrix.tensor_product(&identity);
                one_condition_matrix = one_condition_matrix.tensor_product(&not_matrix);
            }
            else {
                zero_condition_matrix = zero_condition_matrix.tensor_product(&identity);
                one_condition_matrix = one_condition_matrix.tensor_product(&identity);
            }
        }

        Self {
            matrix: Matrix::matrix_addition(zero_condition_matrix, one_condition_matrix).unwrap()
        }
    }

    /**
    Creates a single qubit gate that acts on a quantum register of num_qubits size
    Process of creating such a gate is similar to the multi qubit cnot but with fewer steps
    Tensor the identity matrix with itself num_qubits - 1 times, and in that chain tensor the input gate matrix at the desired qubits position
    (Ex let target_qubit = 2, let num_qubits = 4, let gate = not: ret = I⊗I⊗X⊗I where I is the identity matrix and X is the not matrix)
    */
    pub fn multi_single_qubit_gate(target_qubit: i32, num_qubits: i32, gate: Gate) -> Result<Self, i32> {
        if gate.matrix.rows.len() > 2{
            return Err(-1)
        }
        let mut new_gate_matrix: Matrix = Matrix::new_identity(2);
        let identity: Matrix = Matrix::new_identity(2);
        if target_qubit == 0{
            new_gate_matrix = gate.matrix.clone();
        }

        for i in 1..num_qubits{
            if i == target_qubit {
                new_gate_matrix = new_gate_matrix.tensor_product(&gate.matrix);
            }
            else {
                new_gate_matrix = new_gate_matrix.tensor_product(&identity);
            }
        }

        Ok(Self {matrix: new_gate_matrix})
    }

    pub fn new_multi_h(num_qubits: i32) -> Self {
        let mut matrix: Matrix = Matrix::new_h();
        let h_clone: Matrix = matrix.clone();
        for _ in 0..num_qubits-1{
            matrix = matrix.tensor_product(&h_clone);
        }
        Self {matrix}
    }

    /**
    Apply gate to a quantum register
    Quantum gates are represented by complex matrices
    The state of the register is represented by a kronecker product of 2 element vectors (which represent qubits)
    The kronecker product of n many qubits is a vector with 2^n many elements
    Apply the gate by multiplying the register state vector by the gate matrix
    */
    pub fn apply(&self, register: &mut QuantumRegister) -> Result<i32, i32> {
        let len: usize = self.matrix.rows.len();
        let mut new_state: Vec<Complex<f32>> = vec![Complex::zero(); len];

        // Return an error if there is a mismatch in size
        if len != register.state.len() {
            return Err(-1)
        }

        for i in 0..self.matrix.rows.len(){
            let mut new_val: Complex<f32> = Complex::zero();
            for j in 0..self.matrix.rows[i].len(){
                new_val += self.matrix.rows[i][j] * register.state[j];
            }
            new_state[i] = new_val;
        }

        register.change_state(new_state);
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::E;
    use nalgebra::{ComplexField, RealField};
    use super::*;

    fn print_matrix(matrix: &Matrix){
        for i in 0..matrix.rows.len(){
            print!("[ ");
            for j in 0..matrix.rows[i].len(){
                print!(" {} ", matrix.rows[i][j].re);
            }
            print!(" ]\n");
        }
    }

    fn print_register(register: &QuantumRegister){
        for i in 0..register.state.len(){
            print!("Value at index {}: {}\n", i, register.state[i]);
        }
    }

    #[test]
    fn test_init() {
        let hadamard: Gate = Gate::new_h();
        //print_matrix(hadamard.matrix);

        let pi_gate: Gate = Gate::new_t();
        //print_matrix(pi_gate.matrix);

        let cnot: Gate = Gate::new_cnot();
        print_matrix(&cnot.matrix);
    }

    #[test]
    fn test_apply(){
        let gate:Gate = Gate::new_cnot();
        let qubit1:Qubit = Qubit::new_one_state();
        let qubit2:Qubit = Qubit::new_zero_state();
        let mut register:QuantumRegister = QuantumRegister::new(qubit1);
        register.add(qubit2);
        print_register(&register);
        print!("\n\n");
        match gate.apply(&mut register) {
            Ok(_) => {
                print_register(&register);
            }
            Err(_) => {
                print!("Error");
            }
        }
    }

    #[test]
    fn test_multi_cnot(){
        let gate: Gate = Gate::new_multi_cnot(0,2,3);
        print_matrix(&gate.matrix);

        let q1: Qubit = Qubit::new_zero_state();
        let q2: Qubit = Qubit::new_one_state();
        let q3: Qubit = Qubit::new_zero_state();
        let mut register: QuantumRegister = QuantumRegister::new(q1);

        register.add(q2);
        register.add(q3);

        print_register(&register);
        print!("\n\n");

        match gate.apply(&mut register){
            Ok(_) => {
                print_register(&register)
            }
            Err(_) => {
                print!("ERROR");
            }
        }

    }

    #[test]
    fn test_multi_single(){
        let gate: Gate = Gate::multi_single_qubit_gate(0,3,Gate::new_y()).unwrap();
        let gate1: Gate = Gate::multi_single_qubit_gate(1,3,Gate::new_not()).unwrap();
        let gate2: Gate = Gate::multi_single_qubit_gate(2,3,Gate::new_z()).unwrap();
        let gate3: Gate = Gate::new_multi_h(3);
        print_matrix(&gate.matrix);

        let q1: Qubit = Qubit::new_one_state();
        let q2: Qubit = Qubit::new_one_state();
        let q3: Qubit = Qubit::new_zero_state();
        let mut register: QuantumRegister = QuantumRegister::new(q1);

        register.add(q2);
        register.add(q3);

        print_register(&register);
        print!("\n\n");

        match gate.apply(&mut register){
            Ok(_) => {
                print_register(&register)
            }
            Err(_) => {
                print!("ERROR");
            }
        }
        print!("\n\n");
        match gate1.apply(&mut register){
            Ok(_) => {
                print_register(&register)
            }
            Err(_) => {
                print!("ERROR");
            }
        }
        print!("\n\n");
        match gate2.apply(&mut register){
            Ok(_) => {
                print_register(&register)
            }
            Err(_) => {
                print!("ERROR");
            }
        }
        print!("\n\n");

        match gate3.apply(&mut register){
            Ok(_) => {
                print_register(&register)
            }
            Err(_) => {
                print!("ERROR");
            }
        }
        print!("\n\n");

        let vec: Vec<Qubit> = register.get_qubit_norms();
        for i in 0..vec.len(){
            print!("Qubit State: (x: {}, y: {})\n", vec[i].state.x, vec[i].state.y);
        }



    }

    #[test]
    fn test_multi_h() {
        let gate: Gate = Gate::new_multi_h(2);
        print_matrix(&gate.matrix);

        let q1: Qubit = Qubit::new_zero_state();
        let q2: Qubit = Qubit::new_zero_state();
        let mut register: QuantumRegister = QuantumRegister::new(q1);

        register.add(q2);

        print_register(&register);
        print!("\n\n");

        match gate.apply(&mut register){
            Ok(_) => {
                print_register(&register)
            }
            Err(_) => {
                print!("ERROR");
            }
        }
    }
}