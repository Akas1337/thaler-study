use crate::circuit::{Circuit, Gate, GateType, CircuitLayer};


#[derive(Clone, Debug)]
pub enum CellGateType{
    Add(usize, usize),
    Mul(usize, usize),
    Witness,
}

#[derive(Clone, Debug)]
pub struct Cell{
    index: usize,
    layer_id: usize,
    gate_type: CellGateType,
}

pub struct CircuitBuilder{
    cells: Vec<Cell>,
    n_layer: usize,
    n_input: usize,
    n_output: usize,
}

impl CircuitBuilder {
    pub fn new(cells: Vec<Cell>, n_layer: usize, n_input: usize, n_output: usize) -> Self {
        Self{cells, n_layer, n_input, n_output}
    }

    pub fn build_circuit(&self) -> Circuit {
        let mut cells = self.cells.clone();
        cells.sort_by(|a, b| a.layer_id.cmp(&b.layer_id).then(a.index.cmp(&b.index)));
        let mut layer_count = vec![];
        let mut count = 0usize;
        let mut layer = 0usize;
        let mut ind = 0usize;
        while ind < self.cells.len() {
            let cell = cells[ind].clone();
            if cell.layer_id == layer {
                if cell.index != count {
                    panic!("cell index is not consecutive");
                }
                count += 1;
            } else {
                if count == 0 {
                    panic!("have no cell in layer_{}", layer);
                }
                layer_count.push(count);
                count = 0;
                layer += 1;
                ind -= 1;
            }
            ind += 1;
        }
        if count == 0 {
            panic!("have no cell in layer_{}", layer);
        }
        layer_count.push(count);
        if layer_count[0] != self.n_input || layer_count[layer] != self.n_output {
            panic!("wrong number with input or output");
        }

        let num_inputs = layer_count[0];
        let mut layers = vec![];
        let mut layer_end = cells.len();
        for i in 1usize..self.n_layer {
            let la = self.n_layer - i;
            let mut layer = vec![];
            for j in layer_end - layer_count[la]..layer_end {
                match cells[j].gate_type {
                    CellGateType::Witness => {panic!("exist witness cell in layer_{}", la);},
                    CellGateType::Add(l, r) => {
                        if l >= layer_count[la-1] || r >= layer_count[la-1] {
                            panic!("no enough cell in layer_{}", la-1);
                        }
                        layer.push(Gate::new(GateType::Add, [l, r]))
                    },
                    CellGateType::Mul(l, r) => {
                        if l >= layer_count[la-1] || r >= layer_count[la-1] {
                            panic!("no enough cell in layer_{}", la-1);
                        }
                        layer.push(Gate::new(GateType::Mul, [l, r]))
                    },
                }
            }
            layers.push(CircuitLayer::new(layer));
            layer_end -= layer_count[la];
        }
        Circuit::new(layers, num_inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::{CellGateType, Cell, CircuitBuilder};
    use crate::circuit::{Circuit, Gate, GateType, CircuitLayer};

    #[test]
    fn test_circuit_build() {
        let mut cells = vec![];
        cells.push(Cell {
            index: 0,
            layer_id: 2,
            gate_type: CellGateType::Mul(0, 1),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 2,
            gate_type: CellGateType::Mul(2, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 1,
            gate_type: CellGateType::Mul(0, 0),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 1),
        });cells.push(Cell {
            index: 2,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 2),
        });
        cells.push(Cell {
            index: 3,
            layer_id: 1,
            gate_type: CellGateType::Mul(3, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 1,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 2,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 3,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });

        let builder = CircuitBuilder::new(cells, 3, 4, 2);
        let c = builder.build_circuit();
        let c0 = Circuit::new(vec![CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 1]), Gate::new(GateType::Mul, [2, 3])]),
        CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 0]), Gate::new(GateType::Mul, [1, 1]), Gate::new(GateType::Mul, [1, 2]), Gate::new(GateType::Mul, [3, 3])]),]
        ,4);
        assert_eq!(c, c0);
    }

    #[test]
    #[should_panic(expected = "cell index is not consecutive")]
    fn test_circuit_build_panic1() {
        let mut cells = vec![];
        cells.push(Cell {
            index: 0,
            layer_id: 2,
            gate_type: CellGateType::Mul(0, 1),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 2,
            gate_type: CellGateType::Mul(2, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 1,
            gate_type: CellGateType::Mul(0, 0),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 1),
        });cells.push(Cell {
            index: 2,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 2),
        });
        cells.push(Cell {
            index: 6,
            layer_id: 1,
            gate_type: CellGateType::Mul(3, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 1,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 2,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 3,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });

        let builder = CircuitBuilder::new(cells, 3, 4, 2);
        let _ = builder.build_circuit();
    }

    #[test]
    #[should_panic(expected = "have no cell in layer_2")]
    fn test_circuit_build_panic2() {
        let mut cells = vec![];
        cells.push(Cell {
            index: 0,
            layer_id: 3,
            gate_type: CellGateType::Mul(0, 1),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 3,
            gate_type: CellGateType::Mul(2, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 1,
            gate_type: CellGateType::Mul(0, 0),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 1),
        });cells.push(Cell {
            index: 2,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 2),
        });
        cells.push(Cell {
            index: 3,
            layer_id: 1,
            gate_type: CellGateType::Mul(3, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 1,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 2,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 3,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });

        let builder = CircuitBuilder::new(cells, 3, 4, 2);
        let _ = builder.build_circuit();
    }

    #[test]
    #[should_panic(expected = "wrong number with input or output")]
    fn test_circuit_build_panic3() {
        let mut cells = vec![];
        cells.push(Cell {
            index: 0,
            layer_id: 2,
            gate_type: CellGateType::Mul(0, 1),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 2,
            gate_type: CellGateType::Mul(2, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 1,
            gate_type: CellGateType::Mul(0, 0),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 1),
        });cells.push(Cell {
            index: 2,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 2),
        });
        cells.push(Cell {
            index: 3,
            layer_id: 1,
            gate_type: CellGateType::Mul(3, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 1,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 2,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 3,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });

        let builder = CircuitBuilder::new(cells, 3, 4, 3);
        let _ = builder.build_circuit();
    }

    #[test]
    #[should_panic(expected = "exist witness cell in layer_2")]
    fn test_circuit_build_panic4() {
        let mut cells = vec![];
        cells.push(Cell {
            index: 0,
            layer_id: 2,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 1,
            layer_id: 2,
            gate_type: CellGateType::Mul(2, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 1,
            gate_type: CellGateType::Mul(0, 0),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 1),
        });cells.push(Cell {
            index: 2,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 2),
        });
        cells.push(Cell {
            index: 3,
            layer_id: 1,
            gate_type: CellGateType::Mul(3, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 1,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 2,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 3,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });

        let builder = CircuitBuilder::new(cells, 3, 4, 2);
        let _ = builder.build_circuit();
    }

    #[test]
    #[should_panic(expected = "no enough cell in layer_0")]
    fn test_circuit_build_panic5() {
        let mut cells = vec![];
        cells.push(Cell {
            index: 0,
            layer_id: 2,
            gate_type: CellGateType::Mul(0, 1),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 2,
            gate_type: CellGateType::Mul(2, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 1,
            gate_type: CellGateType::Mul(0, 0),
        });
        cells.push(Cell {
            index: 1,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 8),
        });cells.push(Cell {
            index: 2,
            layer_id: 1,
            gate_type: CellGateType::Mul(1, 2),
        });
        cells.push(Cell {
            index: 3,
            layer_id: 1,
            gate_type: CellGateType::Mul(3, 3),
        });
        cells.push(Cell {
            index: 0,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 1,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 2,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });
        cells.push(Cell {
            index: 3,
            layer_id: 0,
            gate_type: CellGateType::Witness,
        });

        let builder = CircuitBuilder::new(cells, 3, 4, 2);
        let _ = builder.build_circuit();
    }
}