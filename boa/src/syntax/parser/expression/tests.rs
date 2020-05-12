use crate::syntax::{
    ast::node::{BinOp, Local, Node},
    ast::op::{AssignOp, BitOp, CompOp, NumOp},
    parser::tests::check_parser,
};

/// Checks numeric operations
#[test]
fn check_numeric_operations() {
    check_parser(
        "a + b",
        vec![Node::from(BinOp::new(
            NumOp::Add,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a+1",
        vec![Node::from(BinOp::new(
            NumOp::Add,
            Local::from("a"),
            Node::const_node(1),
        ))],
    );
    check_parser(
        "a - b",
        vec![Node::from(BinOp::new(
            NumOp::Sub,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a-1",
        vec![Node::from(BinOp::new(
            NumOp::Sub,
            Local::from("a"),
            Node::const_node(1),
        ))],
    );
    check_parser(
        "a / b",
        vec![Node::from(BinOp::new(
            NumOp::Div,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a/2",
        vec![Node::from(BinOp::new(
            NumOp::Div,
            Local::from("a"),
            Node::const_node(2),
        ))],
    );
    check_parser(
        "a * b",
        vec![Node::from(BinOp::new(
            NumOp::Mul,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a*2",
        vec![Node::from(BinOp::new(
            NumOp::Mul,
            Local::from("a"),
            Node::const_node(2),
        ))],
    );
    check_parser(
        "a ** b",
        vec![Node::from(BinOp::new(
            NumOp::Exp,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a**2",
        vec![Node::from(BinOp::new(
            NumOp::Exp,
            Local::from("a"),
            Node::const_node(2),
        ))],
    );
    check_parser(
        "a % b",
        vec![Node::from(BinOp::new(
            NumOp::Mod,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a%2",
        vec![Node::from(BinOp::new(
            NumOp::Mod,
            Local::from("a"),
            Node::const_node(2),
        ))],
    );
}

// Checks complex numeric operations.
#[test]
fn check_complex_numeric_operations() {
    check_parser(
        "a + d*(b-3)+1",
        vec![Node::from(BinOp::new(
            NumOp::Add,
            Node::from(BinOp::new(
                NumOp::Add,
                Local::from("a"),
                Node::from(BinOp::new(
                    NumOp::Mul,
                    Local::from("d"),
                    Node::from(BinOp::new(
                        NumOp::Sub,
                        Local::from("b"),
                        Node::const_node(3),
                    )),
                )),
            )),
            Node::const_node(1),
        ))],
    );
}

/// Checks bitwise operations.
#[test]
fn check_bitwise_operations() {
    check_parser(
        "a & b",
        vec![Node::from(BinOp::new(
            BitOp::And,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a&b",
        vec![Node::from(BinOp::new(
            BitOp::And,
            Local::from("a"),
            Local::from("b"),
        ))],
    );

    check_parser(
        "a | b",
        vec![Node::from(BinOp::new(
            BitOp::Or,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a|b",
        vec![Node::from(BinOp::new(
            BitOp::Or,
            Local::from("a"),
            Local::from("b"),
        ))],
    );

    check_parser(
        "a ^ b",
        vec![Node::from(BinOp::new(
            BitOp::Xor,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a^b",
        vec![Node::from(BinOp::new(
            BitOp::Xor,
            Local::from("a"),
            Local::from("b"),
        ))],
    );

    check_parser(
        "a << b",
        vec![Node::from(BinOp::new(
            BitOp::Shl,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a<<b",
        vec![Node::from(BinOp::new(
            BitOp::Shl,
            Local::from("a"),
            Local::from("b"),
        ))],
    );

    check_parser(
        "a >> b",
        vec![Node::from(BinOp::new(
            BitOp::Shr,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a>>b",
        vec![Node::from(BinOp::new(
            BitOp::Shr,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
}

/// Checks assignment operations.
#[test]
fn check_assign_operations() {
    check_parser(
        "a += b",
        vec![Node::from(BinOp::new(
            AssignOp::Add,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a -= b",
        vec![Node::from(BinOp::new(
            AssignOp::Sub,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a *= b",
        vec![Node::from(BinOp::new(
            AssignOp::Mul,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a **= b",
        vec![Node::from(BinOp::new(
            AssignOp::Exp,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a /= b",
        vec![Node::from(BinOp::new(
            AssignOp::Div,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a %= b",
        vec![Node::from(BinOp::new(
            AssignOp::Mod,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a &= b",
        vec![Node::from(BinOp::new(
            AssignOp::And,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a |= b",
        vec![Node::from(BinOp::new(
            AssignOp::Or,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a ^= b",
        vec![Node::from(BinOp::new(
            AssignOp::Xor,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a <<= b",
        vec![Node::from(BinOp::new(
            AssignOp::Shl,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a >>= b",
        vec![Node::from(BinOp::new(
            AssignOp::Shr,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a %= 10 / 2",
        vec![Node::from(BinOp::new(
            AssignOp::Mod,
            Local::from("a"),
            Node::from(BinOp::new(
                NumOp::Div,
                Node::const_node(10),
                Node::const_node(2),
            )),
        ))],
    );
}

#[test]
fn check_relational_operations() {
    check_parser(
        "a < b",
        vec![Node::from(BinOp::new(
            CompOp::LessThan,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a > b",
        vec![Node::from(BinOp::new(
            CompOp::GreaterThan,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a <= b",
        vec![Node::from(BinOp::new(
            CompOp::LessThanOrEqual,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "a >= b",
        vec![Node::from(BinOp::new(
            CompOp::GreaterThanOrEqual,
            Local::from("a"),
            Local::from("b"),
        ))],
    );
    check_parser(
        "p in o",
        vec![Node::from(BinOp::new(
            CompOp::In,
            Local::from("p"),
            Local::from("o"),
        ))],
    );
}
