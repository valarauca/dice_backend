use smallvec::SmallVec;

use super::{Graph, ModifyGraph, Operation};

/// Modifications allows for the easy collection of modifications
/// to a graph.
#[derive(Clone)]
pub struct Modifications<E: Clone> {
    data: SmallVec<[Operation<E>; 1]>,
}
impl<E: Clone> Default for Modifications<E> {
    fn default() -> Self {
        Self {
            data: SmallVec::default(),
        }
    }
}
impl<E: Clone> Modifications<E> {
    #[inline]
    pub fn extend<I, T>(&mut self, arg: I)
    where
        Operation<E>: From<T>,
        I: IntoIterator<Item = T>,
    {
        self.data.extend(arg.into_iter().map(Operation::<E>::from))
    }

    /// push an item to the collection
    #[inline]
    pub fn push<T>(&mut self, arg: T)
    where
        Operation<E>: From<T>,
    {
        self.data.push(Operation::from(arg));
    }
}
impl<E: Clone, G: Graph<Expr = E>> ModifyGraph<G> for Modifications<E> {
    fn apply(&self, graph: &mut G) {
        // iterate over items, and apply them all
        for item in self.data.iter() {
            item.apply(graph);
        }
    }
}
