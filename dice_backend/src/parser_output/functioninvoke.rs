use std::fmt;

use super::expression::Expression;

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
