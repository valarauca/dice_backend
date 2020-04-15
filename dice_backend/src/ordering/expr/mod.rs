mod op;
pub use self::op::*;
mod stdlibfunc;
pub use self::stdlibfunc::*;
mod literal;
pub use self::literal::*;
mod order;
pub use self::order::*;
mod hash;
pub use self::hash::*;
mod readtracking;
pub use self::readtracking::*;

use super::super::runner::InlinedExpression;

/// Expressions with their order given
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrderedExpression<'a> {
    StdLibFunc(StdLibFunc<'a>),
    Operation(Op),
    ConstantBool(LiteralBool),
    ConstantInt(LiteralInt),
}
impl<'a> OrderedExpression<'a> {
    /// builds a new expression
    pub fn new(arg: &InlinedExpression<'a>) -> Self {
        match Option::None
            .into_iter()
            .chain(Op::new(arg).map(Self::from))
            .chain(LiteralInt::new(arg).map(Self::from))
            .chain(LiteralBool::new(arg).map(Self::from))
            .chain(StdLibFunc::new(arg).map(Self::from))
            .next()
        {
            Option::None => _unreachable_panic!(),
            Option::Some(result) => result,
        }
    }

    /// returns a list of (own_hash, dependent_hash)
    /// this is for providing strong ordering to the final output
    pub fn get_source_sink(&mut self) -> Vec<(u64,u64)> {
        match self {
            &mut OrderedExpression::StdLibFunc(ref mut func) => {
                let v = func.get_source_sink();
                for (_,item) in v.iter() {
                    func.add_source(*item);
                }
                v
            },
            &mut OrderedExpression::Operation(ref mut op) => {
                let v = op.get_source_sink();
                for (_,item) in v.iter() {
                    op.add_source(*item);
                }
                v
            },
            &mut OrderedExpression::ConstantBool( _ ) => Vec::new(),
            &mut OrderedExpression::ConstantInt( _ ) => Vec::new(),
        }
    }
}
impl<'a> AsRef<Hash> for OrderedExpression<'a> {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Hash {
        match self {
            &OrderedExpression::StdLibFunc(ref func) => func.as_ref(),
            &OrderedExpression::Operation(ref op) => op.as_ref(),
            &OrderedExpression::ConstantBool(ref b) => b.as_ref(),
            &OrderedExpression::ConstantInt(ref i) => i.as_ref(),
        }
    }
}
impl<'a> AsRef<ReadTracking> for OrderedExpression<'a> {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b ReadTracking {
        match self {
            &OrderedExpression::StdLibFunc(ref func) => func.as_ref(),
            &OrderedExpression::Operation(ref op) => op.as_ref(),
            &OrderedExpression::ConstantBool(ref b) => b.as_ref(),
            &OrderedExpression::ConstantInt(ref i) => i.as_ref(),
        }
    }
}
impl<'a> AsMut<ReadTracking> for OrderedExpression<'a> {
    #[inline(always)]
    fn as_mut<'b>(&'b mut self) -> &'b mut ReadTracking {
        match self {
            &mut OrderedExpression::StdLibFunc(ref mut func) => func.as_mut(),
            &mut OrderedExpression::Operation(ref mut op) => op.as_mut(),
            &mut OrderedExpression::ConstantBool(ref mut b) => b.as_mut(),
            &mut OrderedExpression::ConstantInt(ref mut i) => i.as_mut(),
        }
    }
}
impl<'a> ReadTrackingOp for OrderedExpression<'a> { }
impl<'a> AsRef<Ordering> for OrderedExpression<'a> {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Ordering {
        match self {
            &OrderedExpression::StdLibFunc(ref func) => func.as_ref(),
            &OrderedExpression::Operation(ref op) => op.as_ref(),
            &OrderedExpression::ConstantBool(ref b) => b.as_ref(),
            &OrderedExpression::ConstantInt(ref i) => i.as_ref(),
        }
    }
}
impl<'a> AsMut<Ordering> for OrderedExpression<'a> {
    #[inline(always)]
    fn as_mut<'b>(&'b mut self) -> &'b mut Ordering {
        match self {
            &mut OrderedExpression::StdLibFunc(ref mut func) => func.as_mut(),
            &mut OrderedExpression::Operation(ref mut op) => op.as_mut(),
            &mut OrderedExpression::ConstantBool(ref mut b) => b.as_mut(),
            &mut OrderedExpression::ConstantInt(ref mut i) => i.as_mut(),
        }
    }
}
impl<'a> HashOp for OrderedExpression<'a> {}
impl<'a> OrderingOp for OrderedExpression<'a> {}
impl<'a> From<Op> for OrderedExpression<'a> {
    #[inline(always)]
    fn from(arg: Op) -> Self {
        Self::Operation(arg)
    }
}
impl<'a> From<LiteralInt> for OrderedExpression<'a> {
    #[inline(always)]
    fn from(arg: LiteralInt) -> Self {
        Self::ConstantInt(arg)
    }
}
impl<'a> From<LiteralBool> for OrderedExpression<'a> {
    #[inline(always)]
    fn from(arg: LiteralBool) -> Self {
        Self::ConstantBool(arg)
    }
}
impl<'a> From<StdLibFunc<'a>> for OrderedExpression<'a> {
    #[inline(always)]
    fn from(arg: StdLibFunc<'a>) -> Self {
        Self::StdLibFunc(arg)
    }
}
