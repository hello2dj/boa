use crate::syntax::{
    ast::node::{BinOp, Node},
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
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a+1",
        vec![Node::from(BinOp::new(
            NumOp::Add,
            Node::local("a"),
            Node::const_node(1),
        ))],
    );
    check_parser(
        "a - b",
        vec![Node::from(BinOp::new(
            NumOp::Sub,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a-1",
        vec![Node::from(BinOp::new(
            NumOp::Sub,
            Node::local("a"),
            Node::const_node(1),
        ))],
    );
    check_parser(
        "a / b",
        vec![Node::from(BinOp::new(
            NumOp::Div,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a/2",
        vec![Node::from(BinOp::new(
            NumOp::Div,
            Node::local("a"),
            Node::const_node(2),
        ))],
    );
    check_parser(
        "a * b",
        vec![Node::from(BinOp::new(
            NumOp::Mul,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a*2",
        vec![Node::from(BinOp::new(
            NumOp::Mul,
            Node::local("a"),
            Node::const_node(2),
        ))],
    );
    check_parser(
        "a ** b",
        vec![Node::from(BinOp::new(
            NumOp::Exp,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a**2",
        vec![Node::from(BinOp::new(
            NumOp::Exp,
            Node::local("a"),
            Node::const_node(2),
        ))],
    );
    check_parser(
        "a % b",
        vec![Node::from(BinOp::new(
            NumOp::Mod,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a%2",
        vec![Node::from(BinOp::new(
            NumOp::Mod,
            Node::local("a"),
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
                Node::local("a"),
                Node::from(BinOp::new(
                    NumOp::Mul,
                    Node::local("d"),
                    Node::from(BinOp::new(
                        NumOp::Sub,
                        Node::local("b"),
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
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a&b",
        vec![Node::from(BinOp::new(
            BitOp::And,
            Node::local("a"),
            Node::local("b"),
        ))],
    );

    check_parser(
        "a | b",
        vec![Node::from(BinOp::new(
            BitOp::Or,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a|b",
        vec![Node::from(BinOp::new(
            BitOp::Or,
            Node::local("a"),
            Node::local("b"),
        ))],
    );

    check_parser(
        "a ^ b",
        vec![Node::from(BinOp::new(
            BitOp::Xor,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a^b",
        vec![Node::from(BinOp::new(
            BitOp::Xor,
            Node::local("a"),
            Node::local("b"),
        ))],
    );

    check_parser(
        "a << b",
        vec![Node::from(BinOp::new(
            BitOp::Shl,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a<<b",
        vec![Node::from(BinOp::new(
            BitOp::Shl,
            Node::local("a"),
            Node::local("b"),
        ))],
    );

    check_parser(
        "a >> b",
        vec![Node::from(BinOp::new(
            BitOp::Shr,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a>>b",
        vec![Node::from(BinOp::new(
            BitOp::Shr,
            Node::local("a"),
            Node::local("b"),
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
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a -= b",
        vec![Node::from(BinOp::new(
            AssignOp::Sub,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a *= b",
        vec![Node::from(BinOp::new(
            AssignOp::Mul,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a **= b",
        vec![Node::from(BinOp::new(
            AssignOp::Exp,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a /= b",
        vec![Node::from(BinOp::new(
            AssignOp::Div,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a %= b",
        vec![Node::from(BinOp::new(
            AssignOp::Mod,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a &= b",
        vec![Node::from(BinOp::new(
            AssignOp::And,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a |= b",
        vec![Node::from(BinOp::new(
            AssignOp::Or,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a ^= b",
        vec![Node::from(BinOp::new(
            AssignOp::Xor,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a <<= b",
        vec![Node::from(BinOp::new(
            AssignOp::Shl,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a >>= b",
        vec![Node::from(BinOp::new(
            AssignOp::Shr,
            Node::local("a"),
            Node::local("b"),
        ))],
    );
    check_parser(
        "a %= 10 / 2",
        vec![Node::from(BinOp::new(
            AssignOp::Mod,
            Node::local("a"),
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
            Node::Local(String::from("a")),
            Node::Local(String::from("b")),
        ))],
    );
    check_parser(
        "a > b",
        vec![Node::from(BinOp::new(
            CompOp::GreaterThan,
            Node::Local(String::from("a")),
            Node::Local(String::from("b")),
        ))],
    );
    check_parser(
        "a <= b",
        vec![Node::from(BinOp::new(
            CompOp::LessThanOrEqual,
            Node::Local(String::from("a")),
            Node::Local(String::from("b")),
        ))],
    );
    check_parser(
        "a >= b",
        vec![Node::from(BinOp::new(
            CompOp::GreaterThanOrEqual,
            Node::Local(String::from("a")),
            Node::Local(String::from("b")),
        ))],
    );
    check_parser(
        "p in o",
        vec![Node::from(BinOp::new(
            CompOp::In,
            Node::Local(String::from("p")),
            Node::Local(String::from("o")),
        ))],
    );
}
