use super::hash::{Hash, HashOp};
use super::order::{Ordering, OrderingOp};

use super::super::super::parser_output::Operation;
use super::super::super::runner::InlinedExpression;

/// Op contains information about an operation.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Op {
    pub left: u64,
    pub op: Operation,
    pub right: u64,
    order: Ordering,
    hash: Hash,
}
impl Op {
    /// attempts to build an operation from an inlined expression
    pub fn new<'a>(arg: &InlinedExpression<'a>) -> Option<Self> {
        match arg {
            &InlinedExpression::Operation(ref left, ref op, ref right) => {
                let hash = Hash::from(arg);
                Some(Self {
                    left: left.clone(),
                    op: op.clone(),
                    right: right.clone(),
                    order: Ordering::default(),
                    hash: hash,
                })
            }
            _ => None,
        }
    }
}
impl AsRef<Hash> for Op {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Hash {
        &self.hash
    }
}
impl HashOp for Op {}
impl AsRef<Ordering> for Op {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Ordering {
        &self.order
    }
}
impl AsMut<Ordering> for Op {
    #[inline(always)]
    fn as_mut<'b>(&'b mut self) -> &'b mut Ordering {
        &mut self.order
    }
}
impl OrderingOp for Op {}
