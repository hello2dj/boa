//! Array declaration node.

use super::{join_nodes, Node};
use gc::{Finalize, Trace};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An array is an ordered collection of data (either primitive or object depending upon the
/// language).
///
/// Arrays are used to store multiple values in a single variable.
/// This is compared to a variable that can store only one value.
///
/// Each item in an array has a number attached to it, called a numeric index, that allows you
/// to access it. In JavaScript, arrays start at index zero and can be manipulated with various
/// methods.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-ArrayLiteral
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Clone, Debug, Trace, Finalize, PartialEq)]
pub struct ArrayDecl {
    arr: Box<[Node]>,
}

impl ArrayDecl {
    /// Creates an `ArrayDecl` AST node.
    pub fn array_decl<N>(nodes: N) -> Self
    where
        N: Into<Box<[Node]>>,
    {
        Self { arr: nodes.into() }
    }
}

impl AsRef<[Node]> for ArrayDecl {
    fn as_ref(&self) -> &[Node] {
        &self.arr
    }
}

impl<T> From<T> for ArrayDecl
where
    T: Into<Box<[Node]>>,
{
    fn from(decl: T) -> Self {
        Self { arr: decl.into() }
    }
}

impl fmt::Display for ArrayDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[")?;
        join_nodes(f, &self.arr)?;
        f.write_str("]")
    }
}

impl From<ArrayDecl> for Node {
    fn from(arr: ArrayDecl) -> Self {
        Self::ArrayDecl(arr)
    }
}
