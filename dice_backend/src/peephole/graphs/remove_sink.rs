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
    pub fn new<A, B>(from: A, sink: B) -> Self
    where
        Match: From<A>,
        Match: From<B>,
    {
        let from = Match::from(from);
        let sink = Match::from(sink);
        Self { from, sink }
    }
}
impl<G: Graph> ModifyGraph<G> for RemoveSink {
    /// apply this transform to a graph
    fn apply(&self, graph: &mut G) {
        graph.remove_sink(&self.from, &self.sink);
    }
}
