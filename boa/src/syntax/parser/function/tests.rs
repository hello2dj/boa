use crate::syntax::{
    ast::node::{ArrowFunctionDecl, BinOp, FormalParameter, Local, Node},
    ast::op::NumOp,
    parser::tests::check_parser,
};

/// Checks basic function declaration parsing.
#[test]
fn check_basic() {
    check_parser(
        "function foo(a) { return a; }",
        vec![Node::function_decl(
            "foo",
            vec![FormalParameter::new("a", None, false)],
            Node::statement_list(vec![Node::return_node(Node::from(Local::from("a")))]),
        )],
    );
}

/// Checks basic function declaration parsing with automatic semicolon insertion.
#[test]
fn check_basic_semicolon_insertion() {
    check_parser(
        "function foo(a) { return a }",
        vec![Node::function_decl(
            "foo",
            vec![FormalParameter::new("a", None, false)],
            Node::statement_list(vec![Node::return_node(Node::from(Local::from("a")))]),
        )],
    );
}

/// Checks functions with empty returns.
#[test]
fn check_empty_return() {
    check_parser(
        "function foo(a) { return; }",
        vec![Node::function_decl(
            "foo",
            vec![FormalParameter::new("a", None, false)],
            Node::statement_list(vec![Node::Return(None)]),
        )],
    );
}

/// Checks functions with empty returns without semicolon
#[test]
fn check_empty_return_semicolon_insertion() {
    check_parser(
        "function foo(a) { return }",
        vec![Node::function_decl(
            "foo",
            vec![FormalParameter::new("a", None, false)],
            Node::statement_list(vec![Node::Return(None)]),
        )],
    );
}

/// Checks rest operator parsing.
#[test]
fn check_rest_operator() {
    check_parser(
        "function foo(a, ...b) {}",
        vec![Node::function_decl(
            "foo",
            vec![
                FormalParameter::new("a", None, false),
                FormalParameter::new("b", None, true),
            ],
            Node::StatementList(Box::new([])),
        )],
    );
}

/// Checks an arrow function with only a rest parameter.
#[test]
fn check_arrow_only_rest() {
    check_parser(
        "(...a) => {}",
        vec![ArrowFunctionDecl::new(
            vec![FormalParameter::new("a", None, true)],
            Node::StatementList(Box::new([])),
        )
        .into()],
    );
}

/// Checks an arrow function with a rest parameter.
#[test]
fn check_arrow_rest() {
    check_parser(
        "(a, b, ...c) => {}",
        vec![ArrowFunctionDecl::new(
            vec![
                FormalParameter::new("a", None, false),
                FormalParameter::new("b", None, false),
                FormalParameter::new("c", None, true),
            ],
            Node::StatementList(Box::new([])),
        )
        .into()],
    );
}

/// Checks an arrow function with expression return.
#[test]
fn check_arrow() {
    check_parser(
        "(a, b) => { return a + b; }",
        vec![ArrowFunctionDecl::new(
            vec![
                FormalParameter::new("a", None, false),
                FormalParameter::new("b", None, false),
            ],
            Node::statement_list(vec![Node::return_node(Node::from(BinOp::new(
                NumOp::Add,
                Local::from("a"),
                Local::from("b"),
            )))]),
        )
        .into()],
    );
}

/// Checks an arrow function with expression return and automatic semicolon insertion
#[test]
fn check_arrow_semicolon_insertion() {
    check_parser(
        "(a, b) => { return a + b }",
        vec![ArrowFunctionDecl::new(
            vec![
                FormalParameter::new("a", None, false),
                FormalParameter::new("b", None, false),
            ],
            Node::statement_list(vec![Node::return_node(Node::from(BinOp::new(
                NumOp::Add,
                Local::from("a"),
                Local::from("b"),
            )))]),
        )
        .into()],
    );
}

/// Checks arrow function with empty return
#[test]
fn check_arrow_epty_return() {
    check_parser(
        "(a, b) => { return; }",
        vec![ArrowFunctionDecl::new(
            vec![
                FormalParameter::new("a", None, false),
                FormalParameter::new("b", None, false),
            ],
            Node::statement_list(vec![Node::Return(None)]),
        )
        .into()],
    );
}

/// Checks an arrow function with empty return, with automatic semicolon insertion.
#[test]
fn check_arrow_empty_return_semicolon_insertion() {
    check_parser(
        "(a, b) => { return }",
        vec![ArrowFunctionDecl::new(
            vec![
                FormalParameter::new("a", None, false),
                FormalParameter::new("b", None, false),
            ],
            Node::statement_list(vec![Node::Return(None)]),
        )
        .into()],
    );
}
