// example: unary arithmetics
// building a simple interaction system for unary arithmetics using the symbols successor, 0, + and
// x.
// sx + y = s(x + y)
// 0 + y = Y
// sx x y = (x + y) + y
// 0 x y = 0

use crate::cell::{Cell, Label, Port, InteractionNet, Wire};

pub fn successor_cell() -> Cell {
    Cell {
        arity: 1,
        principal_port: Port {
            label: Label::PRINCIPAL,
        },
        auxiliary_ports: vec![Port {
            label: Label::AUXILIAR,
        }],
        label: Label::PRINCIPAL, // You can adjust the label as needed
    }
}

pub fn zero_cell() -> Cell {
    Cell {
        arity: 0,
        principal_port: Port {
            label: Label::PRINCIPAL,
        },
        auxiliary_ports: Vec::new(),
        label: Label::PRINCIPAL, // You can adjust the label as needed
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
        label: Label::PRINCIPAL, // You can adjust the label as needed
    }
}

// to implement interaction, we kind-off need to "slice" the net, reduce the sliced part and connect it together again. Also we need a way to choose which reduction to start with. Reductions are done when two principal ports are connected by a wire.