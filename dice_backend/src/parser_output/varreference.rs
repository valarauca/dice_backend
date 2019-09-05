use std::fmt;

use super::typedata::TypeData;

use super::GetType;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariableReference<'a> {
    pub name: &'a str,
}
impl<'a> fmt::Display for VariableReference<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl<'a> GetType for VariableReference<'a> {
    fn requires_namespace(&self) -> bool {
        true
    }
    fn get_type(&self) -> Result<TypeData, String> {
        Err(format!(
            "variable reference requires a variable lookup to find type data"
        ))
    }
}
