use super::super::super::ordering::OrdTrait;
use super::super::{Match, MatchTrait};

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
    fn add_sink<A, B>(&mut self, expr: &A, new_sink: &B)
    where
        A: MatchTrait,
        B: MatchTrait + Clone;

    /// This will remove the sink specified by `new_sink` from `expr`
    fn remove_sink<A, B>(&mut self, expr: &A, sink_to_remove: &B)
    where
        A: MatchTrait,
        B: MatchTrait;

    /// Updates a source for an expression.
    ///
    /// The ordering of sources is critical, as it represents argument
    /// ordering. Instead of exposing argument indexing, I just expose
    /// a CAS interface.
    ///
    /// For `expr`, the `old` source, will become the `new` source.
    fn compare_and_swap_source<A, B, C>(&mut self, expr: &A, old: &B, new: &C)
    where
        A: MatchTrait,
        B: MatchTrait + Clone,
        C: MatchTrait + Clone;

    /// Removes an expression from the graph.
    ///
    /// No extra work is done to trim edges. Users should emit additional
    /// items to ensure a cooheriant (correct) modification is created.
    fn remove_expr<A>(&mut self, matcher: &A)
    where
        A: MatchTrait;
}
