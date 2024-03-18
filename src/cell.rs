struct Port {}

// a cell has arity >= 0, that has one principal port and n auxiliary_ports
pub struct Cell {
    arity: usize,
    principal_port: Port,
    auxiliary_ports: Vec<Port>,
}

// a net is a graph consisting of a finite number of cells and an extra set of free ports, each
// port being connected to another one by means of a wire.
