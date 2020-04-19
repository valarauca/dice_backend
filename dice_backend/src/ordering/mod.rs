mod consts;
pub use self::consts::{Dice3, Dice3Iter, Dice6, Dice6Iter};
mod datum;
pub use self::datum::Datum;
mod element;
pub use self::element::{Element, Rational};
mod lambda;
pub use self::lambda::{
    coalesce, const_bool, const_int, count, d3, d6, filter, join, len, sum, Chain, Coalesce,
    CoalesceChain, CoalesceCombinator, Combinator, Init, LambdaKind, Iter,
};
mod coll;
