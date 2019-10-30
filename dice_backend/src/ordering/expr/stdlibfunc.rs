
use super::order::{Ordering,OrderingOp};
use super::hash::{Hash,HashOp};

use super::super::super::runner::InlinedExpression;

/// StdLibFunc represents a stdlibi invocation
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Debug)]
pub struct StdLibFunc<'a> {
    pub name: &'a str,
    pub arg: Box<[u64]>,
    order: Ordering,
    hash: Hash, 
}
impl<'a> StdLibFunc<'a> {
    pub fn new(arg: &InlinedExpression<'a>) -> Option<Self> {
        match arg {
            &InlinedExpression::StdLibFunc(name, ref args) => {
                let hash = Hash::from(arg);
                Some(Self{
                    name, hash,
                    arg: args.clone(),
                    order: Ordering::default(),
                })
            },
            _ => None,
        }
    }
}
impl<'a> AsRef<Hash> for StdLibFunc<'a> {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Hash {
        &self.hash
    }
}
impl<'a> HashOp for StdLibFunc<'a> { }
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
impl<'a> OrderingOp for StdLibFunc<'a> { }
