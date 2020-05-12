//! Statement list node.

use super::{join_nodes, FormalParameter, Node};
use gc::{Finalize, Trace};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// List of statements.
///
/// Similar to `Node::Block` but without the braces.
///
/// More information:
///  - [ECMAScript reference][spec]
///
/// [spec]: https://tc39.es/ecma262/#prod-StatementList
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct StatementList {
    functions: Box<[FunctionDecl]>,
    statements: Box<[Node]>,
}

impl StatementList {
    /// Creates a new statement list.
    pub(crate) fn new<F, S>(functions: F, statements: S) -> Self
    where
        F: Into<Box<[FunctionDecl]>>,
        S: Into<Box<[Node]>>,
    {
        Self {
            functions: functions.into(),
            statements: statements.into(),
        }
    }

    /// Implements the display formatting with indentation.
    pub(super) fn display(&self, f: &mut fmt::Formatter<'_>, indentation: usize) -> fmt::Result {
        // Print the functions first.
        for function in self.functions.iter() {
            function.display(f, indentation)?;
            writeln!(f);
        }

        // Print statements
        for node in self.statements.iter() {
            node.display(f, indentation + 1)?;

            match node {
                Node::Block(_)
                | Node::If(_, _, _)
                | Node::Switch(_, _, _)
                | Node::WhileLoop(_, _) => {}
                _ => write!(f, ";")?,
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Display for StatementList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f, 0)
    }
}

/// The `var` statement declares a variable, optionally initializing it to a value.
///
/// var declarations, wherever they occur, are processed before any code is executed. This is
/// called hoisting, and is discussed further below.
///
/// The scope of a variable declared with var is its current execution context, which is either
/// the enclosing function or, for variables declared outside any function, global. If you
/// re-declare a JavaScript variable, it will not lose its value.
///
/// Assigning a value to an undeclared variable implicitly creates it as a global variable (it
/// becomes a property of the global object) when the assignment is executed.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-VariableStatement
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/var
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct VarDecl {
    vars: Box<[Box<str>]>,
}

impl<T> From<T> for VarDecl
where
    T: Into<Box<[Box<str>]>>,
{
    fn from(list: T) -> Self {
        Self { vars: list.into() }
    }
}

impl fmt::Display for VarDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.vars.is_empty() {
            write!(f, "var ")?;
            join_nodes(f, &self.vars)
        } else {
            Ok(())
        }
    }
}

/// The `function` declaration (function statement) defines a function with the specified
/// parameters.
///
/// A function created with a function declaration is a `Function` object and has all the
/// properties, methods and behavior of `Function`.
///
/// A function can also be created using an expression (see [function expression][func_expr]).
///
/// By default, functions return `undefined`. To return any other value, the function must have
/// a return statement that specifies the value to return.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#sec-terms-and-definitions-function
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/function
/// [func_expr]: ../enum.Node.html#variant.FunctionExpr
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct FunctionDecl {
    name: Box<str>,
    parameters: Box<[FormalParameter]>,
    body: StatementList,
}

impl FunctionDecl {
    /// Implements the display formatting with indentation.
    fn display(&self, f: &mut fmt::Formatter<'_>, indentation: usize) -> fmt::Result {
        write!(f, "function {} (", self.name)?;
        join_nodes(f, &self.parameters)?;
        f.write_str(") {{")?;

        self.body.display(f, indentation + 1);

        writeln!(f, "}}")
    }
}
