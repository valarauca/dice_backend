use super::traits::{Graph, ModifyGraph};
use super::Match;

/// Remove will 'remove' one more items from a graph.
///
/// It is not _directly_ responsible for rewriting edges,
/// this should be specified by other operations.
#[repr(packed)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Remover {
    data: Match,
}
impl Remover {
    /// build from what ever we can build a matcher from
    #[inline]
    pub fn new<T>(data: T) -> Self
    where
        Match: From<T>,
    {
        let data = Match::from(data);
        Self { data }
    }
}
impl ModifyGraph for Remover {
    /// apply this transform to a graph
    fn apply<G>(&self, graph: &mut G)
    where
        G: Graph,
    {
        graph.remove_expr(&self.data);
    }
}
