use interaction_combinators::cell::{Cell, InteractionNet, Label, Port, Wire};

fn main() {
    let mut net = InteractionNet::new();

    let cell1 = Cell {
        arity: 2,
        principal_port: Port {
            label: Label::PRINCIPAL,
        },
        auxiliary_ports: vec![
            Port {
                label: Label::AUXILIAR,
            },
            Port {
                label: Label::AUXILIAR,
            },
        ],
        label: Label::PRINCIPAL,
    };
    net.add_cell(cell1);

    let cell2 = Cell {
        arity: 1,
        principal_port: Port {
            label: Label::PRINCIPAL,
        },
        auxiliary_ports: vec![],
        label: Label::AUXILIAR,
    };
    net.add_cell(cell2);

    let free_port1 = Port {
        label: Label::PRINCIPAL,
    };
    net.add_free_port(free_port1);

    let free_port2 = Port {
        label: Label::AUXILIAR,
    };
    net.add_free_port(free_port2);

    net.connect_ports(0, 0, 1, 0);
    net.connect_ports(0, 1, 1, 0);

    println!("{:?}", net);
}
