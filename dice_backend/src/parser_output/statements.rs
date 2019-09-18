use std::fmt;

use super::statement::Statement;

/// Statements are a collection of operations
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Statements<'a> {
    pub data: Box<[Statement<'a>]>,
}
impl<'a> fmt::Display for Statements<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in self.data.iter() {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}
impl<'a> Statements<'a> {
    #[inline(always)]
    pub fn new(arg: Vec<(Statement<'a>, &'a str)>) -> Statements<'a> {
        #[inline(always)]
        fn mapper<'a>(tup: (Statement<'a>, &'a str)) -> Statement<'a> {
            tup.0
        }
        let collect: Vec<Statement<'a>> = arg.into_iter().map(mapper).collect();
        Statements {
            data: collect.into_boxed_slice(),
        }
    }
}
