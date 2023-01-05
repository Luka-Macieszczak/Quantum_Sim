use crate::Gates::Gate;
use crate::QuantumRegister::QuantumRegister;
use num_traits::One;
use num_traits::Zero;
use nalgebra::Complex;
use crate::Qubit::Qubit;

pub(crate) struct QuantumCircuit {
    gates: Vec<Gate>,
    num_qubits: i32
}

impl QuantumCircuit {
    pub fn new(gate: Gate, num_qubits: i32) -> Self {
        Self {gates: vec![gate], num_qubits}
    }

    pub fn add_gate(&mut self, gate: Gate){
        self.gates.push(gate);
    }

    pub fn run(&self, register: &mut QuantumRegister) {
        for i  in 0..self.gates.len(){
            match self.gates[i].apply(register) {
                Ok(_) => {
                    continue
                }
                Err(_) => {
                    print!("Error in applying gate\n")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_register(register: &QuantumRegister){
        for i in 0..register.state.len(){
            print!("Value at index {}: {}\n", i, register.state[i]);
        }
    }

    #[test]
    fn test_init() {
        let num_qubits: i32 = 4;
        let h: Gate = Gate::new_multi_h(num_qubits);
        let gate: Gate = Gate::new_multi_cnot(0, 3, num_qubits);
        let gate1: Gate = Gate::new_multi_cnot(1, 3, num_qubits);
        let mut circuit: QuantumCircuit = QuantumCircuit::new(h, num_qubits);
        //let gate1: Gate = Gate::multi_single_qubit_gate(2, num_qubits, Gate::new_identity()).unwrap();
        //let gate2: Gate = Gate::multi_single_qubit_gate(1, num_qubits, Gate::new_identity()).unwrap();
        let gate2: Gate = Gate::new_multi_cnot(2,3,num_qubits);
        circuit.add_gate(gate);
        circuit.add_gate(gate1);
        circuit.add_gate(gate2);

        let q1: Qubit = Qubit::new_one_state();
        let q2: Qubit = Qubit::new_zero_state();
        let q3: Qubit = Qubit::new_zero_state();
        let q4: Qubit = Qubit::new_zero_state();
        let mut register: QuantumRegister = QuantumRegister::new(q1);
        register.add(q2);
        register.add(q3);
        register.add(q4);
        print_register(&register);
        print!("\n\n");
        circuit.run(&mut register);
        print_register(&register);

    }


}