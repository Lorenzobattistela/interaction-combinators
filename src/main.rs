use interaction_combinators::cell::{Cell, InteractionNet, Port};

fn main() {
    // Create a new interaction net
    let mut net = InteractionNet::new();

    // Create a couple of cells
    let cell1 = Cell {
        arity: 2,
        principal_port: Port { connected_to: None },
        auxiliary_ports: vec![Port { connected_to: None }, Port { connected_to: None }],
    };

    let cell2 = Cell {
        arity: 1,
        principal_port: Port { connected_to: None },
        auxiliary_ports: vec![Port { connected_to: None }],
    };

    // Add the cells to the net
    net.add_cell(cell1);
    net.add_cell(cell2);

    // Connect ports between cells
    net.connect_ports(0, 0, 1, 0);
    net.connect_ports(0, 1, 1, 0);

    // Print the net to see the connections
    println!("{:#?}", net);
}
