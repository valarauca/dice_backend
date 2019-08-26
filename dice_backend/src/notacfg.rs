
//! credit to: https://pp.ipd.kit.edu/uploads/publikationen/braun13cc.pdf
//! 
//! but as an uninformed observer by querying the call chain lazily you are
//! just building a CFG, just an emergent one of causality.

pub struct NameCollection<'a> {
    names: HashMap<(&'a str)>,

}

enum Name<'a> {
    Var(Expr<'a>),

}

