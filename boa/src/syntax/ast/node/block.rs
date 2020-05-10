//! Block AST node.

use super::Node;
use gc::{Finalize, Trace};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A `block` statement (or compound statement in other languages) is used to group zero or
/// more statements.
///
/// The block statement is often called compound statement in other languages.
/// It allows you to use multiple statements where JavaScript expects only one statement.
/// Combining statements into blocks is a common practice in JavaScript. The opposite behavior
/// is possible using an empty statement, where you provide no statement, although one is
/// required.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-BlockStatement
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/block
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct Block {
    hoistable: Box<[Node]>,
    statements: Box<[Node]>,
}

impl Block {
    /// Creates a `Block` AST node.
    pub fn new<H, S>(hoistable: H, statements: S) -> Self
    where
        H: Into<Box<[Node]>>,
        S: Into<Box<[Node]>>,
    {
        Self {
            hoistable: hoistable.into(),
            statements: statements.into(),
        }
    }

    /// Gets the list of hoistable statements.
    pub fn hoistable(&self) -> &[Node] {
        &self.hoistable
    }

    /// Gets the list of non-hoistable statements.
    pub fn statements(&self) -> &[Node] {
        &self.statements
    }

    /// Implements the display formatting with indentation.
    pub(super) fn display(&self, f: &mut fmt::Formatter<'_>, indentation: usize) -> fmt::Result {
        writeln!(f, "{{")?;
        for node in self.hoistable.iter().chain(self.statements.iter()) {
            node.display(f, indentation + 1)?;

            match node {
                Node::Block(_)
                | Node::If(_, _, _)
                | Node::Switch(_, _, _)
                | Node::FunctionDecl(_, _, _)
                | Node::WhileLoop(_, _)
                | Node::StatementList(_) => {}
                _ => write!(f, ";")?,
            }
            writeln!(f)?;
        }
        write!(f, "{}}}", "    ".repeat(indentation))
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

impl From<Block> for Node {
    fn from(block: Block) -> Self {
        Self::Block(block)
    }
}
