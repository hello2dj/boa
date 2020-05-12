//! Block statement parsing tests.

use crate::syntax::{
    ast::{
        node::{Assign, Block, Local, Node},
        op::UnaryOp,
    },
    parser::tests::check_parser,
};

/// Helper function to check a block.
// TODO: #[track_caller]: https://github.com/rust-lang/rust/issues/47809
fn check_block(js: &str, block: Block) {
    check_parser(js, vec![Node::from(block)]);
}

#[test]
fn empty() {
    check_block("{}", Block::from(vec![]));
}

#[test]
fn non_empty() {
    check_block(
        r"{
            var a = 10;
            a++;
        }",
        Block::from(vec![
            Node::var_decl(vec![("a".into(), Some(Node::const_node(10)))]),
            Node::unary_op(UnaryOp::IncrementPost, Node::from(Local::from("a"))),
        ]),
    );

    check_block(
        r"{
            function hello() {
                return 10
            }

            var a = hello();
            a++;
        }",
        Block::from(vec![
            Node::function_decl(
                "hello",
                vec![],
                Node::statement_list(vec![Node::return_node(Node::const_node(10))]),
            ),
            Node::var_decl(vec![(
                "a".into(),
                Some(Node::call(Node::from(Local::from("hello")), vec![])),
            )]),
            Node::unary_op(UnaryOp::IncrementPost, Node::from(Local::from("a"))),
        ]),
    );
}

#[test]
fn hoisting() {
    check_block(
        r"{
            var a = hello();
            a++;

            function hello() { return 10 }
        }",
        Block::from(vec![
            Node::function_decl(
                "hello",
                vec![],
                Node::statement_list(vec![Node::return_node(Node::const_node(10))]),
            ),
            Node::var_decl(vec![(
                "a".into(),
                Some(Node::call(Node::from(Local::from("hello")), vec![])),
            )]),
            Node::unary_op(UnaryOp::IncrementPost, Node::from(Local::from("a"))),
        ]),
    );

    check_block(
        r"{
            a = 10;
            a++;

            var a;
        }",
        Block::from(vec![
            Node::var_decl(vec![("a".into(), None)]),
            Node::from(Assign::new(Local::from("a"), Node::const_node(10))),
            Node::unary_op(UnaryOp::IncrementPost, Node::from(Local::from("a"))),
        ]),
    );
}
