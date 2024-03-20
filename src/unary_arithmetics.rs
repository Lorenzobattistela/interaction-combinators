// example: unary arithmetics
// building a simple interaction system for unary arithmetics using the symbols successor, 0, + and
// x.
// sx + y = s(x + y)
// 0 + y = Y
// sx x y = (x + y) + y
// 0 x y = 0

use crate::cell::{Cell, InteractionNet, Label, Port, Wire};

pub fn successor_cell() -> Cell {
    Cell {
        arity: 1,
        principal_port: Port {
            label: Label::PRINCIPAL,
        },
        auxiliary_ports: vec![Port {
            label: Label::AUXILIAR,
        }],
        label: Label::SUC, // You can adjust the label as needed
    }
}

pub fn zero_cell() -> Cell {
    Cell {
        arity: 0,
        principal_port: Port {
            label: Label::PRINCIPAL,
        },
        auxiliary_ports: Vec::new(),
        label: Label::ZERO,
    }
}

pub fn plus_cell() -> Cell {
    Cell {
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
        label: Label::SUM,
    }
}

// to implement interaction, we kind-off need to "slice" the net, reduce the sliced part and connect it together again. Also we need a way to choose which reduction to start with. Reductions are done when two principal ports are connected by a wire.
// then we need to get the connections of the ports involved on the reduce process, because we'll
// need to connect different ports with the correct wires
pub fn reduce_sum_suc(sum_net: InteractionNet) -> InteractionNet {
    let possible_reductions = sum_net.possible_reductions();
    let mut res = InteractionNet::new();
    for &reduction in possible_reductions.iter() {
        let wire = &sum_net.wires[reduction];
        println!("{:?}", wire);
        if let Some((from_cell_index, from_port_index)) = wire.from {
            if let Some((to_cell_index, to_port_index)) = wire.to {
                let from_port = match sum_net.get_port(from_cell_index, from_port_index) {
                    Some(from_port) => from_port,
                    None => panic!("Something went wrong"),
                };
                let to_port = match sum_net.get_port(to_cell_index, to_port_index) {
                    Some(to_port) => to_port,
                    None => panic!("Something went wrong"),
                };

                println! {"{:?}", to_port};
                println!("{:?}", from_port);

                let connections = sum_net.get_all_connections(from_cell_index, from_port_index);
                println!("{:?}", connections);
            }
        }
    }
    res
}



