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

use super::super::runner::InlinedExpression;

/// Expressions with their order given
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrderedExpression<'a> {
    StdLibFunc(StdLibFunc<'a>),
    Operation(Op),
    Constant(LiteralValue<'a>),
}
impl<'a> OrderedExpression<'a> {
    /// builds a new expression
    pub fn new(arg: &InlinedExpression<'a>) -> Self {
        match Option::None
            .into_iter()
            .chain(Op::new(arg).map(Self::from))
            .chain(LiteralValue::new(arg).map(Self::from))
            .chain(StdLibFunc::new(arg).map(Self::from))
            .next()
        {
            Option::None => _unreachable_panic!(),
            Option::Some(result) => result,
        }
    }
}
impl<'a> AsRef<Hash> for OrderedExpression<'a> {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Hash {
        match self {
            &OrderedExpression::StdLibFunc(ref func) => func.as_ref(),
            &OrderedExpression::Operation(ref op) => op.as_ref(),
            &OrderedExpression::Constant(ref con) => con.as_ref(),
        }
    }
}
impl<'a> AsRef<Ordering> for OrderedExpression<'a> {
    #[inline(always)]
    fn as_ref<'b>(&'b self) -> &'b Ordering {
        match self {
            &OrderedExpression::StdLibFunc(ref func) => func.as_ref(),
            &OrderedExpression::Operation(ref op) => op.as_ref(),
            &OrderedExpression::Constant(ref con) => con.as_ref(),
        }
    }
}
impl<'a> AsMut<Ordering> for OrderedExpression<'a> {
    #[inline(always)]
    fn as_mut<'b>(&'b mut self) -> &'b mut Ordering {
        match self {
            &mut OrderedExpression::StdLibFunc(ref mut func) => func.as_mut(),
            &mut OrderedExpression::Operation(ref mut op) => op.as_mut(),
            &mut OrderedExpression::Constant(ref mut con) => con.as_mut(),
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
impl<'a> From<LiteralValue<'a>> for OrderedExpression<'a> {
    #[inline(always)]
    fn from(arg: LiteralValue<'a>) -> Self {
        Self::Constant(arg)
    }
}
impl<'a> From<StdLibFunc<'a>> for OrderedExpression<'a> {
    #[inline(always)]
    fn from(arg: StdLibFunc<'a>) -> Self {
        Self::StdLibFunc(arg)
    }
}
