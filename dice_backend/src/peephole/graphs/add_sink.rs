use super::traits::{Graph, ModifyGraph};
use super::Match;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AddSink {
    to: Match,
    sink: Match,
}
impl AddSink {
    #[inline(always)]
    pub fn new<A, B>(to: A, sink: B) -> Self
    where
        Match: From<A>,
        Match: From<B>,
    {
        let to = Match::from(to);
        let sink = Match::from(sink);
        Self { to, sink }
    }
}
impl<G: Graph> ModifyGraph<G> for AddSink {
    /// apply this transform to a graph
    fn apply(&self, graph: &mut G) {
        graph.add_sink(&self.to, &self.sink);
    }
}
