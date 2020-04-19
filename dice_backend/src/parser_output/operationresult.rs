use std::fmt;

use super::expression::Expression;
use super::operation::Operation;
use super::typedata::TypeData;

use super::GetType;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationResult<'a> {
    pub left: Box<Expression<'a>>,
    pub op: Operation,
    pub right: Box<Expression<'a>>,
}
impl<'a> fmt::Display for OperationResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} {})", self.left, self.op, self.right)
    }
}
impl<'a> GetType for OperationResult<'a> {
    fn requires_namespace(&self) -> bool {
        self.left.requires_namespace() || self.right.requires_namespace()
    }

    fn get_type(&self) -> Result<TypeData, String> {
        if self.requires_namespace() {
            return Err(format!(
                "interior expressions require namespacing for this operation to complete"
            ));
        }
        let left = self.left.get_type()?;
        let right = self.right.get_type()?;
        panic!("todo")
    }
}
