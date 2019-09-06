use std::fmt;

use super::expression::Expression;
use super::typedata::TypeData;

/// ConstantDeclaration is when a constant is declared globally.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstantDeclaration<'a> {
    pub name: &'a str,
    pub kind: TypeData,
    pub expr: Expression<'a>,
}
impl<'a> fmt::Display for ConstantDeclaration<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "const {}: {} = {} ;", self.name, self.kind, self.expr)
    }
}
