use std::fmt;

use super::expression::Expression;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerminalExpression<'a> {
    pub expr: Expression<'a>,
}
impl<'a> fmt::Display for TerminalExpression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {};", self.expr)
    }
}
