use super::super::Match;
use super::traits::{Graph, ModifyGraph};

/// Insert a node into the graph
#[derive(Clone)]
pub struct Inserter<E: Clone> {
    expr: E,
}
impl<E: Clone> Inserter<E> {
    #[inline(always)]
    pub fn new(expr: E) -> Self {
        Self { expr }
    }
}
impl<E, G> ModifyGraph<G> for Inserter<E>
where
    E: Clone,
    G: Graph<Expr = E>,
{
    /// apply this transformation
    fn apply(&self, graph: &mut G) {
        graph.insert(self.expr.clone());
    }
}
