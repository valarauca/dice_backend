use super::hash::{Hash, HashOp};
use super::order::{Ordering, OrderingOp};
use super::readtracking::{ReadTracking, ReadTrackingOp};

use super::super::super::parser_output::Literal;
use super::super::super::runner::InlinedExpression;

/// LiteralValue contains a literal value
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LiteralInt {
    pub lit: i32,
    ordering: Ordering,
    hash: Hash,
    read: ReadTracking,
}
impl LiteralInt {
    pub fn new<'a>(arg: &InlinedExpression<'a>) -> Option<Self> {
        match arg {
            &InlinedExpression::ConstantInt(ref i) => {
                let hash = Hash::from(arg);
                Some(Self {
                    lit: *i,
                    hash,
                    ordering: Ordering::default(),
                    read: ReadTracking::default(),
                })
            },
            _ => None,
        }
    }
}
impl AsRef<ReadTracking> for LiteralInt {
    #[inline(always)]
    fn as_ref(&self) -> &ReadTracking {
        &self.read
    }
}
impl AsMut<ReadTracking> for LiteralInt {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut ReadTracking {
        &mut self.read
    }
}
impl AsRef<Hash> for LiteralInt {
    #[inline(always)]
    fn as_ref(&self) -> &Hash {
        &self.hash
    }
}
impl AsRef<Ordering> for LiteralInt {
    #[inline(always)]
    fn as_ref(&self) -> &Ordering {
        &self.ordering
    }
}
impl AsMut<Ordering> for LiteralInt {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Ordering {
        &mut self.ordering
    }
}
impl HashOp for LiteralInt { }
impl OrderingOp for LiteralInt { }
impl ReadTrackingOp for LiteralInt { }

/// LiteralBool contains a literal boolean value
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LiteralBool {
    pub lit: bool,
    ordering: Ordering,
    hash: Hash,
    read: ReadTracking,
}
impl LiteralBool {
    pub fn new<'a>(arg: &InlinedExpression<'a>) -> Option<Self> {
        match arg {
            &InlinedExpression::ConstantBool(ref i) => {
                let hash = Hash::from(arg);
                Some(Self {
                    lit: *i,
                    ordering: Ordering::default(),
                    read: ReadTracking::default(),
                    hash,
                })
            },
            _ => None,
        }
    }
}
impl AsRef<ReadTracking> for LiteralBool {
    #[inline(always)]
    fn as_ref(&self) -> &ReadTracking {
        &self.read
    }
}
impl AsMut<ReadTracking> for LiteralBool {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut ReadTracking {
        &mut self.read
    }
}
impl AsRef<Hash> for LiteralBool {
    #[inline(always)]
    fn as_ref(&self) -> &Hash {
        &self.hash
    }
}
impl AsRef<Ordering> for LiteralBool {
    #[inline(always)]
    fn as_ref(&self) -> &Ordering {
        &self.ordering
    }
}
impl AsMut<Ordering> for LiteralBool {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Ordering {
        &mut self.ordering
    }
}
impl HashOp for LiteralBool { }
impl OrderingOp for LiteralBool { }
impl ReadTrackingOp for LiteralBool { }


