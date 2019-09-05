use std::fmt;

use super::operation::Operation;
use super::Expression;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationResult<'a> {
    pub left: Box<Expression<'a>>,
    pub op: Operation,
    pub right: Box<Expression<'a>>,
}
