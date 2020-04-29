use super::traits::{Graph, ModifyGraph};
use super::Match;

#[repr(packed)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RemoveSink {
    from: Match,
    sink: Match,
}
impl RemoveSink {
    #[inline(always)]
    pub fn new(from: Match, sink: Match) -> Self {
        Self { from, sink }
    }
}
impl ModifyGraph for RemoveSink {
    /// apply this transform to a graph
    fn apply<G>(&self, graph: &mut G)
    where
        G: Graph,
    {
        graph.remove_sink(&self.from, &self.sink);
    }
}
