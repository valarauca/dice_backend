use super::literal::Literal;
use super::typedata::TypeData;

use super::GetType;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiteralValue<'a> {
    pub lit: Literal<'a>,
}
impl<'a> GetType for LiteralValue<'a> {
    fn get_type(&self) -> Result<TypeData, String> {
        self.lit.get_type()
    }
}
