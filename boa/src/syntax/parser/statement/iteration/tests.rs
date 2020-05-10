use crate::syntax::{
    ast::node::{BinOp, Block, Node},
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
            Node::from(Block::new(
                vec![],
                vec![BinOp::new(AssignOp::Add, Node::local("a"), Node::const_node(1)).into()],
            )),
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
            Node::var_decl(vec![(String::from("i"), Some(Node::const_node(0)))]),
            Node::do_while_loop(
                Node::from(Block::new(
                    vec![],
                    vec![Node::call(
                        Node::get_const_field(Node::local("console"), "log"),
                        vec![Node::const_node("hello")],
                    )],
                )),
                Node::from(BinOp::new(
                    CompOp::LessThan,
                    Node::unary_op(UnaryOp::IncrementPost, Node::local("i")),
                    Node::const_node(10),
                )),
            ),
            Node::call(
                Node::get_const_field(Node::local("console"), "log"),
                vec![Node::const_node("end")],
            ),
        ],
    );
}
