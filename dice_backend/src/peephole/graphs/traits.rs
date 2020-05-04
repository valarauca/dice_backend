use super::super::super::ordering::OrdTrait;
use super::Match;

/// ModifyGraph handles the application of a change to a graph.
pub trait ModifyGraph<G: Graph> {
    fn apply(&self, graph: &mut G);
}

/// Graph is a 'generalized' API for interacting with the graph.
///
///
/// It means the underlying graph-rewrite operations don't need
/// to understand what collection they're necessarily interacting
/// with.
pub trait Graph {
    type Expr;

    /// Insert a node into the graph
    fn insert(&mut self, expr: Self::Expr);

    /// This will add the tuple specified by `new_sink` to `expr`.
    ///
    /// No extra work is done.
    fn add_sink(&mut self, expr: &Match, new_sink: &Match);

    /// This will remove the sink specified by `new_sink` from `expr`
    fn remove_sink(&mut self, expr: &Match, sink_to_remove: &Match);

    /// Updates a source for an expression.
    ///
    /// The ordering of sources is critical, as it represents argument
    /// ordering. Instead of exposing argument indexing, I just expose
    /// a CAS interface.
    ///
    /// For `expr`, the `old` source, will become the `new` source.
    fn compare_and_swap_source(&mut self, expr: &Match, old: &Match, new: &Match);

    /// Removes an expression from the graph.
    ///
    /// No extra work is done to trim edges. Users should emit additional
    /// items to ensure a cooheriant (correct) modification is created.
    fn remove_expr(&mut self, matcher: &Match);
}
