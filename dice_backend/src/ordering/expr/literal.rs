
use super::order::{Ordering,OrderingOp};
use super::hash::{Hash,HashOp};

use super::super::super::parser_output::{Literal};
use super::super::super::runner::InlinedExpression;

/// LiteralValue contains a literal value
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub struct LiteralValue<'a> {
    pub lit: Literal<'a>,
    order: Ordering,
    hash: Hash,
}
impl<'a> LiteralValue<'a> {

    /// attempts to build a literal value from an inlined expression
    pub fn new(arg: &InlinedExpression<'a>) -> Option<Self> {
        match arg {
            &InlinedExpression::Constant(ref lit) => {
               let hash = Hash::from(arg);
               Some(Self {
                   lit: lit.clone(),
                   order: Ordering::default(),
                   hash,
               })
            },
            _ => None,
        }
    }
}
impl<'a> AsRef<Hash> for LiteralValue<'a> {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Hash {
        &self.hash
    }
}
impl<'a> HashOp for LiteralValue<'a> { }
impl<'a> AsRef<Ordering> for LiteralValue<'a> {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Ordering {
        &self.order
    }
}
impl<'a> AsMut<Ordering> for LiteralValue<'a> {
    #[inline(always)]
    fn as_mut<'b>(&'b mut self) -> &'b mut Ordering {
        &mut self.order
    }
}
impl<'a> OrderingOp for LiteralValue<'a> { }

