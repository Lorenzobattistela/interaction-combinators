#[derive(Debug, Clone, Copy)]
pub enum Label {
    PRINCIPAL,
    AUXILIAR,
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
    wires: Vec<Wire>,
    free_ports: Vec<Port>,
}

impl InteractionNet {
    pub fn new() -> InteractionNet {
        InteractionNet {
            cells: Vec::new(),
            wires: Vec::new(),
            free_ports: Vec::new(), // Initializing free ports collection
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

    pub fn interact_successor_plus(&mut self, successor_index: usize, plus_index: usize) {
        // Get references to the successor and plus cells
        let successor_cell = &self.cells[successor_index];
        let plus_cell = &self.cells[plus_index];

        // Ensure the arity of the successor cell is 1
        if successor_cell.arity != 1 {
            panic!("Successor cell arity must be 1.");
        }

        // Ensure the arity of the plus cell is 2
        if plus_cell.arity != 2 {
            panic!("Plus cell arity must be 2.");
        }

        // Ensure the principal port of the successor cell is connected to the principal port of the plus cell
        self.connect_ports(
            successor_index,
            0, // Principal port index of successor
            plus_index,
            0, // Principal port index of plus
        );

        // Create a new plus cell
        let new_plus_cell = Cell {
            arity: 2,
            principal_port: Port {
                label: Label::AUXILIAR,
            }, // Aux port of s becomes principal port of new plus cell
            auxiliary_ports: vec![
                Port {
                    label: Label::AUXILIAR,
                },
                Port {
                    label: Label::PRINCIPAL,
                },
            ], // Aux port 1 is a free port, Aux port 2 is a successor cell
            label: Label::PRINCIPAL, // You can adjust the label as needed
        };

        // Add the new plus cell to the interaction net
        self.add_cell(new_plus_cell);

        // Index of the newly added plus cell
        let new_plus_index = self.cells.len() - 1;

        // Connect aux port 2 of the resulting plus cell to a successor cell
        let new_successor_index = self.cells.len(); // Index of the new successor cell
        let new_successor_cell = crate::unary_arithmetics::successor_cell(); // Create a new successor cell
        self.add_cell(new_successor_cell);
        self.connect_ports(
            new_plus_index,
            1, // Aux port 2 index of the resulting plus cell
            new_successor_index,
            0, // Principal port index of the new successor cell
        );

        // Connect the aux port of the new successor cell to aux port 2 of the plus cell
        self.connect_ports(
            new_successor_index,
            0, // Principal port index of the new successor cell
            plus_index,
            1, // Aux port 2 index of the plus cell
        );

        // Convert the aux port 1 of the resulting plus cell to a free port
        self.add_free_port(plus_cell.auxiliary_ports[0].clone());
    }
}
