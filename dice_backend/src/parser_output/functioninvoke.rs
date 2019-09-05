use std::fmt;

use super::expression::Expression;
use super::typedata::TypeData;

use super::GetType;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionInvocation<'a> {
    pub name: &'a str,
    pub args: Box<[Expression<'a>]>,
}
impl<'a> fmt::Display for FunctionInvocation<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}( ", self.name)?;
        let last_arg = self.args.len() - 1;
        for (pos, arg) in self.args.iter().enumerate() {
            if last_arg == pos {
                write!(f, "{}", arg)?;
            } else {
                write!(f, "{}, ", arg)?;
            }
        }
        write!(f, " )")
    }
}
impl<'a> GetType for FunctionInvocation<'a> {
    fn requires_namespace(&self) -> bool {
        true
    }

    fn get_type(&self) -> Result<TypeData, String> {
        Err(format!(
            "FunctionInvocation requires its name be looked up prior to resolving typing"
        ))
    }
}
