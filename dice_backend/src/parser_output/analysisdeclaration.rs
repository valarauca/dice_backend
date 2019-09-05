use std::fmt;

use super::expression::Expression;

/// AnalysisDeclaraction is one of the last top level structures.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnalysisDeclaration<'a> {
    pub expr: Expression<'a>,
}
impl<'a> fmt::Display for AnalysisDeclaration<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "analyze {};\n", self.expr)
    }
}
