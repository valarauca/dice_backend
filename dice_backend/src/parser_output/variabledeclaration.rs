use std::fmt;

use super::expression::Expression;
use super::typedata::TypeData;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariableDeclaration<'a> {
    pub name: &'a str,
    pub kind: TypeData,
    pub expr: Expression<'a>,
}
impl<'a> fmt::Display for VariableDeclaration<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let {}: {} = {}", self.name, self.kind, self.expr)
    }
}
