use interaction_combinators::{
    cell::{Cell, InteractionNet, Label, Port, Wire},
    unary_arithmetics::{
        has_cell_with_label, has_ports_with_label, has_zero_and_sum_ports, plus_cell,
        reduce_sum_suc, reduce_sum_zero, zero_cell,
    },
};

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

    let possible_reds = net.possible_reductions();
    println!("Possible reduction in wire: {:?}", possible_reds);
    let first_red = possible_reds[0];
    let wire_to_reduce = &net.wires[first_red];
    println!("Wire to reduce: {:?}", wire_to_reduce);

    // reduce_sum_suc(net);

    let sum_port = Port { label: Label::SUM };

    let zero_port = Port { label: Label::ZERO };

    let has_sum_zero_ports = has_zero_and_sum_ports(&sum_port, &zero_port);
    println!("Has zero and sum ports: {}", has_sum_zero_ports);

    // checking same res using dif function
    let has_ports_label = has_ports_with_label(Label::SUM, Label::ZERO, &sum_port, &zero_port);
    println!("Has zero and sum ports general: {}", has_ports_label);

    let mut zero_sum_net = InteractionNet::new();
    let zero_cell = zero_cell();
    let sum_cell = plus_cell();

    zero_sum_net.add_cell(zero_cell.clone());
    zero_sum_net.add_cell(sum_cell.clone());

    zero_sum_net.connect_ports(0, 0, 1, 0);

    println!("Net: {:?}", zero_sum_net);

    let has_cell_w_label = has_cell_with_label(Label::SUM, Label::ZERO, &zero_cell, &sum_cell);
    println!("Has cell with labels 0 and sum: {}", has_cell_w_label);

    reduce_sum_zero(net);
}
