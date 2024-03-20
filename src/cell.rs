#[derive(Debug, Clone, Copy)]
pub enum Label {
    PRINCIPAL,
    AUXILIAR,
    SUM,
    SUC,
    ZERO,
}

#[derive(Debug, Clone, Copy)]
// where connected_to is the cell index on a net and the port index on a cell port
pub struct Port {
    pub label: Label,
}

// a cell has arity >= 0, that has one principal port and n auxiliary_ports
#[derive(Debug)]
pub struct Cell {
    pub arity: usize,
    pub principal_port: Port,
    pub auxiliary_ports: Vec<Port>,
    pub label: Label,
}

#[derive(Debug)]
pub struct Wire {
    pub from: Option<(usize, usize)>,
    pub to: Option<(usize, usize)>,
}

// a net is a graph consisting of a finite number of cells and an extra set of free ports, each
// port being connected to another one by means of a wire.
#[derive(Debug)]
pub struct InteractionNet {
    cells: Vec<Cell>,
    pub wires: Vec<Wire>,
    free_ports: Vec<Port>,
}

impl InteractionNet {
    pub fn new() -> InteractionNet {
        InteractionNet {
            cells: Vec::new(),
            wires: Vec::new(),
            free_ports: Vec::new(),
        }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn add_free_port(&mut self, port: Port) {
        self.free_ports.push(port);
    }

    pub fn add_wire(&mut self, wire: Wire) {
        self.wires.push(wire);
    }

    pub fn connect_ports(
        &mut self,
        from_cell: usize,
        from_port: usize,
        to_cell: usize,
        to_port: usize,
    ) {
        let new_wire = Wire {
            from: Some((from_cell, from_port)),
            to: Some((to_cell, to_port)),
        };
        self.add_wire(new_wire);
    }

    pub fn get_port(&self, cell_index: usize, port_index: usize) -> Option<&Port> {
        println!("cell_idx: {}, port_idx: {}", cell_index, port_index);
        if let Some(cell) = self.cells.get(cell_index) {
            if port_index == 0 {
                Some(&cell.principal_port)
            } else {
                println!("{:?}", cell.auxiliary_ports);
                cell.auxiliary_ports.get(port_index - 1)
            }
        } else {
            None // Return None if the cell index is out of bounds
        }
    }

    pub fn get_all_connections(&self, cell_index: usize, port_index: usize) -> Vec<&Wire> {
        self.wires
            .iter()
            .filter(|wire| {
                if let Some((from_cell_idx, from_port_idx)) = wire.from {
                    if from_cell_idx == cell_index && from_port_idx == port_index {
                        return true;
                    }
                }
                if let Some((to_cell_idx, to_port_idx)) = wire.to {
                    if to_cell_idx == cell_index && to_port_idx == port_index {
                        return true;
                    }
                }
                false
            })
            .collect()
    }

    // returns all the wires that have two principal ports connected
    pub fn possible_reductions(&self) -> Vec<usize> {
        let mut reductions = Vec::new();
        for (i, wire) in self.wires.iter().enumerate() {
            if let (Some((_from_cell, from_port)), Some((_to_cell, to_port))) = (wire.from, wire.to)
            {
                // port 0 is principal port
                if from_port == 0 as usize && to_port == 0 as usize {
                    reductions.push(i);
                }
            }
        }
        reductions
    }
}
