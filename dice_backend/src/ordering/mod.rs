//mod coll;

mod consts;
pub use self::consts::{Dice3, Dice3Iter, Dice6, Dice6Iter};
mod datum;
pub use self::datum::Datum;
mod element;
pub use self::element::{Element, ElementFilter, ElementIterator};
mod lambda;
