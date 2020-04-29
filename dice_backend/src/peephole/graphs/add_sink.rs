use super::traits::{Graph, ModifyGraph};
use super::Match;

#[repr(packed)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AddSink {
    to: Match,
    sink: Match,
}
impl AddSink {
    #[inline(always)]
    pub fn new(to: Match, sink: Match) -> Self {
        Self { to, sink }
    }
}
impl ModifyGraph for AddSink {
    /// apply this transform to a graph
    fn apply<G>(&self, graph: &mut G)
    where
        G: Graph,
    {
        graph.add_sink(&self.to, &self.sink);
    }
}
