use smallvec::SmallVec;

use super::{Graph, ModifyGraph, Operation};

/// Modifications allows for the easy collection of modifications
/// to a graph.
#[derive(Clone, Default)]
pub struct Modifications {
    data: SmallVec<[Operation; 1]>,
}
impl Modifications {
    #[inline]
    pub fn extend<I, T>(&mut self, arg: I)
    where
        Operation: From<T>,
        I: IntoIterator<Item = T>,
    {
        self.data.extend(arg.into_iter().map(Operation::from))
    }

    /// push an item to the collection
    #[inline]
    pub fn push<T>(&mut self, arg: T)
    where
        Operation: From<T>,
    {
        self.data.push(Operation::from(arg));
    }
}
impl ModifyGraph for Modifications {
    fn apply<G>(&self, graph: &mut G)
    where
        G: Graph,
    {
        // iterate over items, and apply them all
        for item in self.data.iter() {
            item.apply(graph);
        }
    }
}
