use super::traits::{Graph, ModifyGraph};
use super::Match;

/// Insert a node into the graph
#[derive(Clone)]
pub struct Insert<E: Clone> {
    expr: E,
}
impl<E: Clone> Insert<E> {
    #[inline(always)]
    pub fn new(expr: E) -> Self {
        Self { expr }
    }
}
impl<E, G> ModifyGraph<G> for Insert<E>
where
    E: Clone,
    G: Graph<Expr = E>,
{
    /// apply this transformation
    fn apply(&self, graph: &mut G) {
        graph.insert(self.expr.clone());
    }
}
