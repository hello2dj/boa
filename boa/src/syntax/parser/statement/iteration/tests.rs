use crate::syntax::{
    ast::node::{BinOp, Block, Local, Node},
    ast::op::{AssignOp, CompOp, UnaryOp},
    parser::tests::check_parser,
};

/// Checks do-while statement parsing.
#[test]
fn check_do_while() {
    check_parser(
        r#"do {
            a += 1;
        } while (true)"#,
        vec![Node::do_while_loop(
            Node::from(Block::from(vec![Node::from(BinOp::new(
                AssignOp::Add,
                Local::from("a"),
                Node::const_node(1),
            ))])),
            Node::const_node(true),
        )],
    );
}

// Checks automatic semicolon insertion after do-while.
#[test]
fn check_do_while_semicolon_insertion() {
    check_parser(
        r#"var i = 0;
        do {console.log("hello");} while(i++ < 10) console.log("end");"#,
        vec![
            Node::var_decl(vec![("i".into(), Some(Node::const_node(0)))]),
            Node::do_while_loop(
                Node::from(Block::from(vec![Node::call(
                    Node::get_const_field(Node::from(Local::from("console")), "log"),
                    vec![Node::const_node("hello")],
                )])),
                Node::from(BinOp::new(
                    CompOp::LessThan,
                    Node::unary_op(UnaryOp::IncrementPost, Node::from(Local::from("i"))),
                    Node::const_node(10),
                )),
            ),
            Node::call(
                Node::get_const_field(Node::from(Local::from("console")), "log"),
                vec![Node::const_node("end")],
            ),
        ],
    );
}
