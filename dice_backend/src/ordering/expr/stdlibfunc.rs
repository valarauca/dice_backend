use super::hash::{Hash, HashOp};
use super::order::{Ordering, OrderingOp};
use super::readtracking::{ReadTracking, ReadTrackingOp};

use super::super::super::runner::InlinedExpression;

/// StdLibFunc represents a stdlibi invocation
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StdLibFunc<'a> {
    pub name: &'a str,
    pub arg: Box<[u64]>,
    order: Ordering,
    hash: Hash,
    read: ReadTracking,
}
impl<'a> StdLibFunc<'a> {
    pub fn new(arg: &InlinedExpression<'a>) -> Option<Self> {
        match arg {
            &InlinedExpression::StdLibFunc(name, ref args) => {
                let hash = Hash::from(arg);
                Some(Self {
                    name,
                    hash,
                    arg: args.clone(),
                    order: Ordering::default(),
                    read: ReadTracking::default(),
                })
            }
            _ => None,
        }
    }

    /// returns a tuple of (own_hash, arg_hash) for ordering
    pub fn get_source_sink(&self) -> Vec<(u64,u64)> {
        self.arg
            .iter()
            .map(|arg_hash| (self.get_hash(), *arg_hash))
            .collect()
    }
}
impl<'a> AsRef<ReadTracking> for StdLibFunc<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &ReadTracking {
        &self.read
    }
}
impl<'a> AsMut<ReadTracking> for StdLibFunc<'a> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut ReadTracking {
        &mut self.read
    }
}
impl<'a> ReadTrackingOp for StdLibFunc<'a> { }
impl<'a> AsRef<Hash> for StdLibFunc<'a> {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Hash {
        &self.hash
    }
}
impl<'a> HashOp for StdLibFunc<'a> {}
impl<'a> AsRef<Ordering> for StdLibFunc<'a> {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Ordering {
        &self.order
    }
}
impl<'a> AsMut<Ordering> for StdLibFunc<'a> {
    #[inline(always)]
    fn as_mut<'b>(&'b mut self) -> &'b mut Ordering {
        &mut self.order
    }
}
impl<'a> OrderingOp for StdLibFunc<'a> {}
