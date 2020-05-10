use crate::syntax::{
    ast::node::{Block, Node},
    parser::tests::check_parser,
};

#[test]
fn check_inline() {
    check_parser(
        "while (true) break;",
        vec![Node::while_loop(Node::const_node(true), Node::Break(None))],
    );
}

#[test]
fn check_new_line() {
    check_parser(
        "while (true)
            break;",
        vec![Node::while_loop(Node::const_node(true), Node::Break(None))],
    );
}

#[test]
fn check_inline_block_semicolon_insertion() {
    check_parser(
        "while (true) {break}",
        vec![Node::while_loop(
            Node::const_node(true),
            Node::from(Block::new(vec![], vec![Node::Break(None)])),
        )],
    );
}

#[test]
fn check_new_line_semicolon_insertion() {
    check_parser(
        "while (true) {
            break test
        }",
        vec![Node::while_loop(
            Node::const_node(true),
            Node::from(Block::new(vec![], vec![Node::break_node("test")])),
        )],
    );
}

#[test]
fn check_inline_block() {
    check_parser(
        "while (true) {break;}",
        vec![Node::while_loop(
            Node::const_node(true),
            Node::from(Block::new(vec![], vec![Node::Break(None)])),
        )],
    );
}

#[test]
fn check_new_line_block() {
    check_parser(
        "while (true) {
            break test;
        }",
        vec![Node::while_loop(
            Node::const_node(true),
            Node::from(Block::new(vec![], vec![Node::break_node("test")])),
        )],
    );
}

#[test]
fn check_new_line_block_empty() {
    check_parser(
        "while (true) {
            break;
        }",
        vec![Node::while_loop(
            Node::const_node(true),
            Node::from(Block::new(vec![], vec![Node::Break(None)])),
        )],
    );
}

#[test]
fn check_new_line_block_empty_semicolon_insertion() {
    check_parser(
        "while (true) {
            break
        }",
        vec![Node::while_loop(
            Node::const_node(true),
            Node::from(Block::new(vec![], vec![Node::Break(None)])),
        )],
    );
}
