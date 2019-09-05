use super::expression::Expression;
use super::typedata::TypeData;

/// ConstantDeclaration is when a constant is declared globally.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstantDeclaration<'a> {
    pub name: &'a str,
    pub kind: TypeData,
    pub expr: Expression<'a>,
}
