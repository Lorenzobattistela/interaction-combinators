#[derive(Debug)]
// where connected_to is the cell index on a net and the port index on a cell port
pub struct Port {
    pub connected_to: Option<(usize, usize)>,
}

// a cell has arity >= 0, that has one principal port and n auxiliary_ports
#[derive(Debug)]
pub struct Cell {
    pub arity: usize,
    pub principal_port: Port,
    pub auxiliary_ports: Vec<Port>,
}

#[derive(Debug)]
pub struct Wire {
    from: Option<(usize, usize)>,
    to: Option<(usize, usize)>,
}

// a net is a graph consisting of a finite number of cells and an extra set of free ports, each
// port being connected to another one by means of a wire.
#[derive(Debug)]
pub struct InteractionNet {
    cells: Vec<Cell>,
}

impl InteractionNet {
    pub fn new() -> InteractionNet {
        InteractionNet { cells: Vec::new() }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn connect_ports(
        &mut self,
        from_cell: usize,
        from_port: usize,
        to_cell: usize,
        to_port: usize,
    ) {
        if let Some(from_port) = self.cells[from_cell].auxiliary_ports.get_mut(from_port) {
            from_port.connected_to = Some((to_cell, to_port));
        }
        if let Some(to_port) = self.cells[to_cell].auxiliary_ports.get_mut(to_port) {
            to_port.connected_to = Some((from_cell, from_port));
        }
    }
}
