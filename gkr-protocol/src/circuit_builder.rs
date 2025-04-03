use crate::circuit::{Circuit, Gate, GateType, CircuitLayer};
use std::collections::{HashSet, HashMap};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CellGateType{
    Add(usize, usize),
    Mul(usize, usize),
    Witness,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    DuplicateGate,
    IllegalGate,
}

#[derive(Clone, Debug)]
pub struct Cell{
    index: usize,
    layer_id: usize,
    gate_type: CellGateType,
}

pub struct CircuitBuilder{
    cells: Vec<Cell>,
    gatehashset: HashSet<CellGateType>,
    n_layer: usize,
    n_input: usize,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        Self{cells: vec![], gatehashset: HashSet::new(), n_layer: 0, n_input: 0}
    }

    pub fn apply_witness(&mut self) -> usize {
        let idx = self.cells.len();
        let cell = Cell {
            index: idx,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        };
        self.cells.push(cell);
        if self.n_layer == 0 {
            self.n_layer = 1
        };
        self.n_input += 1;
        idx
    }

    pub fn append_add_gate(&mut self, left: usize, right: usize) -> Result<usize, BuildError> {
        let gt = CellGateType::Add(left, right);
        if self.gatehashset.contains(&gt) {
            Err(BuildError::DuplicateGate)
        } else if self.cells[left].layer_id != self.cells[right].layer_id {
            Err(BuildError::IllegalGate)
        } else {
            let idx = self.cells.len();
            let layer = self.cells[left].layer_id + 1;
            if layer == self.n_layer {
                self.n_layer += 1
            }
            let cell = Cell {
                index: idx,
                layer_id: layer,
                gate_type: gt.clone(),
            };
            self.gatehashset.insert(gt);
            self.cells.push(cell);
            Ok(idx)
        }
    }

    pub fn append_mul_gate(&mut self, left: usize, right: usize) -> Result<usize, BuildError> {
        let gt = CellGateType::Mul(left, right);
        if self.gatehashset.contains(&gt) {
            Err(BuildError::DuplicateGate)
        } else if self.cells[left].layer_id != self.cells[right].layer_id {
            Err(BuildError::IllegalGate)
        } else {
            let idx = self.cells.len();
            let layer = self.cells[left].layer_id + 1;
            if layer == self.n_layer {
                self.n_layer += 1
            }
            let cell = Cell {
                index: idx,
                layer_id: layer,
                gate_type: gt.clone(),
            };
            self.gatehashset.insert(gt);
            self.cells.push(cell);
            Ok(idx)
        }
    }

    pub fn build_circuit(&self) -> Circuit {
        let mut queue = vec![];
        for cell in self.cells.clone() {
            if cell.layer_id == self.n_layer - 1 {
                queue.push(cell.clone())
            }
        }

        let mut layers = vec![];
        let mut hs = HashSet::new();
        let mut layer_index = HashMap::new();
        for _ in 1usize..self.n_layer {
            let mut cells = vec![];
            let n = queue.len();
            for i in 0usize..n {
                let cell = queue[i].clone();
                cells.push(cell.clone());
                let (l, r) = match cell.gate_type {
                    CellGateType::Add(x, y) => (x, y),
                    CellGateType::Mul(x, y) => (x, y),
                    CellGateType::Witness => panic!("witness only in layer_0")
                };
                if !hs.contains(&l) {
                    hs.insert(l);
                    queue.push(self.cells[l].clone())
                }
                if !hs.contains(&r) {
                    hs.insert(r);
                    queue.push(self.cells[r].clone())
                }
            }

            let newlen = queue.len();
            queue = queue[n..newlen].to_vec();
            queue.sort_by(|a, b| a.index.cmp(&b.index));
            for i in 0usize..queue.len() {
                layer_index.insert(queue[i].index, i);
            }

            let mut layer = vec![];
            for cell in cells {
                match cell.gate_type {
                    CellGateType::Add(l, r) => {
                        layer.push(Gate::new(GateType::Add, [layer_index[&l], layer_index[&r]]));
                    },
                    CellGateType::Mul(l, r) => {
                        layer.push(Gate::new(GateType::Mul, [layer_index[&l], layer_index[&r]]));
                    },
                    CellGateType::Witness => panic!("witness only in layer_0")
                }
            }
            layers.push(CircuitLayer::new(layer));
        }

        let num_inputs = queue.len();
        Circuit::new(layers, num_inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::{CircuitBuilder, BuildError};
    use crate::circuit::{Circuit, Gate, GateType, CircuitLayer};

    //normal circuit check
    #[test]
    fn test_circuit_build() {
        let mut builder = CircuitBuilder::new();
        let w0 = builder.apply_witness();
        let w1 = builder.apply_witness();
        let w2 = builder.apply_witness();
        let w3 = builder.apply_witness();
        let v0 = builder.append_mul_gate(w0, w0).unwrap();
        let v1 = builder.append_mul_gate(w1, w1).unwrap();
        let v2 = builder.append_mul_gate(w1, w2).unwrap();
        let v3 = builder.append_mul_gate(w3, w3).unwrap();
        let _ = builder.append_mul_gate(v0, v1);
        let _ = builder.append_mul_gate(v2, v3);

        let c = builder.build_circuit();
        let c0 = Circuit::new(vec![CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 1]), Gate::new(GateType::Mul, [2, 3])]),
        CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 0]), Gate::new(GateType::Mul, [1, 1]), Gate::new(GateType::Mul, [1, 2]), Gate::new(GateType::Mul, [3, 3])]),]
        ,4);
        assert_eq!(c, c0);
    }

    //change cell order and gate type
    #[test]
    fn test_circuit_build2() {
        let mut builder = CircuitBuilder::new();
        let w0 = builder.apply_witness();
        let v0 = builder.append_add_gate(w0, w0).unwrap();
        let w1 = builder.apply_witness();
        let v1 = builder.append_add_gate(w1, w1).unwrap();
        let _ = builder.append_mul_gate(v0, v1);
        let w2 = builder.apply_witness();
        let v2 = builder.append_mul_gate(w1, w2).unwrap();
        let w3 = builder.apply_witness();
        let v3 = builder.append_add_gate(w3, w3).unwrap();
        let _ = builder.append_add_gate(v2, v3);

        let c = builder.build_circuit();
        let c0 = Circuit::new(vec![CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 1]), Gate::new(GateType::Add, [2, 3])]),
        CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 0]), Gate::new(GateType::Add, [1, 1]), Gate::new(GateType::Mul, [1, 2]), Gate::new(GateType::Add, [3, 3])]),]
        ,4);
        assert_eq!(c, c0);
    }

    //Some cells not contribute to outputs will be ignore
    #[test]
    fn test_circuit_build3() {
        let mut builder = CircuitBuilder::new();
        let w0 = builder.apply_witness();
        let w1 = builder.apply_witness();
        let w2 = builder.apply_witness();
        let w3 = builder.apply_witness();
        let w4 = builder.apply_witness(); //ignored
        let v0 = builder.append_mul_gate(w0, w0).unwrap();
        let v1 = builder.append_mul_gate(w1, w1).unwrap();
        let v1_1 = builder.append_mul_gate(w0, w2).unwrap(); //ignored
        let v2 = builder.append_mul_gate(w1, w2).unwrap();
        let v3 = builder.append_mul_gate(w3, w3).unwrap();
        let _ = builder.append_mul_gate(v0, v1);
        let v5 = builder.append_mul_gate(w1, w4).unwrap(); //ignored
        let _ = builder.append_mul_gate(v2, v3);

        let c = builder.build_circuit();
        let c0 = Circuit::new(vec![CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 1]), Gate::new(GateType::Mul, [2, 3])]),
        CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 0]), Gate::new(GateType::Mul, [1, 1]), Gate::new(GateType::Mul, [1, 2]), Gate::new(GateType::Mul, [3, 3])]),]
        ,4);
        assert_eq!(c, c0);
    }

    //error check
    #[test]
    fn test_circuit_build_error1() {
        let mut builder = CircuitBuilder::new();
        let w0 = builder.apply_witness();
        let w1 = builder.apply_witness();
        let w2 = builder.apply_witness();
        let w3 = builder.apply_witness();
        let v0 = builder.append_mul_gate(w0, w0).unwrap();
        let v1 = builder.append_mul_gate(w1, w1).unwrap();
        let v2 = builder.append_mul_gate(w1, w2).unwrap();
        let v3 = builder.append_mul_gate(w3, w3).unwrap();
        let _ = builder.append_mul_gate(v0, v1);
        let _ = builder.append_mul_gate(v2, v3);
        let err = builder.append_mul_gate(w0, w0).unwrap_err();
        assert_eq!(err, BuildError::DuplicateGate);
    }

    //error check
    #[test]
    fn test_circuit_build_error2() {
        let mut builder = CircuitBuilder::new();
        let w0 = builder.apply_witness();
        let w1 = builder.apply_witness();
        let w2 = builder.apply_witness();
        let w3 = builder.apply_witness();
        let v0 = builder.append_mul_gate(w0, w0).unwrap();
        let v1 = builder.append_mul_gate(w1, w1).unwrap();
        let v2 = builder.append_mul_gate(w1, w2).unwrap();
        let v3 = builder.append_mul_gate(w3, w3).unwrap();
        let _ = builder.append_mul_gate(v0, v1);
        let _ = builder.append_mul_gate(v2, v3);
        let err = builder.append_mul_gate(w2, v3).unwrap_err();
        assert_eq!(err, BuildError::IllegalGate);
    }

}