use super::traits::{Graph, ModifyGraph};
use super::Match;

#[repr(packed)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SwapSource {
    expr: Match,
    old: Match,
    new: Match,
}
impl SwapSource {
    #[inline]
    pub fn new<A, B, C>(expr: A, old: B, new: C) -> Self
    where
        Match: From<A>,
        Match: From<B>,
        Match: From<C>,
    {
        let expr = Match::from(expr);
        let old = Match::from(old);
        let new = Match::from(new);
        Self { expr, old, new }
    }
}
impl ModifyGraph for SwapSource {
    /// apply this transform to a graph
    fn apply<G>(&self, graph: &mut G)
    where
        G: Graph,
    {
        graph.compare_and_swap_source(&self.expr, &self.old, &self.new);
    }
}
