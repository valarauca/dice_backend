use std::fmt;

use super::literal::Literal;
use super::typedata::TypeData;

use super::GetType;

/// LiteralValue is a midly unnecessary wrapper around a literal value.
/// This type exists so the layout of the `Expression` enum is consistent
/// such that it only holds structures, which contain values.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiteralValue<'a> {
    pub lit: Literal<'a>,
}
impl<'a> GetType for LiteralValue<'a> {
    fn requires_namespace(&self) -> bool {
        false
    }

    fn get_type(&self) -> Result<TypeData, String> {
        self.lit.get_type()
    }
}
impl<'a> fmt::Display for LiteralValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.lit.fmt(f)
    }
}
