use std::fmt;

use super::typedata::TypeData;
use super::Statements;

/// ConstantDeclaration is when a constant is declared globally.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionDeclaration<'a> {
    pub name: &'a str,
    pub stdlib: bool,
    pub args: Box<[(&'a str, TypeData)]>,
    pub ret: TypeData,
    pub body: Statements<'a>,
}
impl<'a> fmt::Display for FunctionDeclaration<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "func {}(", self.name)?;
        let last_arg = self.args.len() - 1;
        for (pos, arg) in self.args.iter().enumerate() {
            if last_arg == pos {
                write!(f, "{}: {}", arg.0, arg.1)?;
            } else {
                write!(f, "{}: {}, ", arg.0, arg.1)?;
            }
        }
        write!(f, " )")
    }
}
