use crate::Gates::Gate;
use crate::QuantumRegister::QuantumRegister;
use crate::Qubit::Qubit;

pub struct QuantumCircuit {
    gates: Vec<Gate>,
    num_qubits: i32
}

impl QuantumCircuit {
    pub fn new(gate: Gate, num_qubits: i32) -> Self {
        Self {gates: vec![gate], num_qubits}
    }

    pub fn add_gate(&mut self, gate: Gate) -> &mut QuantumCircuit {
        self.gates.push(gate);
        self
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

    pub fn run_with_tracking(&self, register: &mut QuantumRegister) -> Vec<Vec<Qubit>> {
        let mut ret: Vec<Vec<Qubit>> = vec![];
        for i  in 0..self.gates.len(){
            ret.push(register.get_qubit_norms());
            match self.gates[i].apply(register) {
                Ok(_) => {
                    continue
                }
                Err(_) => {
                    print!("Error in applying gate\n")
                }
            }
        }
        ret
    }

    pub fn order_finding(input: i32) -> Self {
        let num_qubits: i32 = 2 * ((input as f32).log2().floor()) as i32;
        let hadamard: Gate = Gate::new_custom(
            Gate::new_multi_h(num_qubits/2).matrix.tensor_product(
                &Gate::new_identity(num_qubits/2).matrix
            ));
        Self{gates: vec![hadamard], num_qubits}
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
        let h1: Gate = Gate::multi_single_qubit_gate(0,num_qubits,Gate::new_h()).unwrap();
        let h2: Gate = Gate::multi_single_qubit_gate(1,num_qubits,Gate::new_h()).unwrap();
        let h3: Gate = Gate::multi_single_qubit_gate(0,num_qubits,Gate::new_t()).unwrap();
        let gate: Gate = Gate::new_multi_cnot(0, 3, num_qubits);
        let gate1: Gate = Gate::new_multi_cnot(1, 3, num_qubits);

        let mut circuit: QuantumCircuit = QuantumCircuit::new(h1, num_qubits);

        //let gate1: Gate = Gate::multi_single_qubit_gate(2, num_qubits, Gate::new_identity()).unwrap();
        //let gate2: Gate = Gate::multi_single_qubit_gate(1, num_qubits, Gate::new_identity()).unwrap();
        let gate2: Gate = Gate::new_multi_cnot(2,3,num_qubits);
        circuit.add_gate(h2)
            .add_gate(h3)
            .add_gate(gate)
            .add_gate(gate1)
            .add_gate(gate2);

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
        let vec = circuit.run_with_tracking(&mut register);
        print_register(&register);


        let qubits: Vec<Qubit> = register.get_qubit_norms();
        print!("\n\n");

        for i in 0..qubits.len(){
            print!("Qubit State: (x: {}, y: {})\n", qubits[i].state.x, qubits[i].state.y);
        }

        let res = register.measure();
        print!("\n\n");
        print_register(&register);
        print!("\n\nRes: {}", res);
        print!("\n\n");
        for i in 0..vec.len(){
            print!("State: {}\n", i);
            for j in 0..vec[i].len(){
                print!("Qubit state: {}, {}\n", vec[i][j].state.x, vec[i][j].state.y);
            }
            print!("\n\n");
        }


    }

    #[test]
    fn order_test() {
        let circuit: QuantumCircuit = QuantumCircuit::order_finding(4);
        print!("Quincy")
    }


}