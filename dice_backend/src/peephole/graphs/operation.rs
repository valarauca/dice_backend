use super::{AddSink, Graph, Inserter, ModifyGraph, RemoveSink, Remover, SwapSource};

/// Operation is an enum over the permitted graph transformations.
///
/// This enum exists so it is trivial to "replay" modifications to
/// a graph.
///
/// Its trait implementations also allows for the collection of its
/// arguments to be pretty simple.
#[derive(Clone)]
pub enum Operation<E: Clone> {
    AddSink(AddSink),
    RemoveSink(RemoveSink),
    SwapSource(SwapSource),
    RemoveExpr(Remover),
    Inserter(Inserter<E>),
}
impl<E: Clone> From<Inserter<E>> for Operation<E> {
    #[inline(always)]
    fn from(insert: Inserter<E>) -> Self {
        Operation::Inserter(insert)
    }
}
impl<E: Clone> From<AddSink> for Operation<E> {
    #[inline(always)]
    fn from(add_sink: AddSink) -> Self {
        Operation::AddSink(add_sink)
    }
}
impl<E: Clone> From<RemoveSink> for Operation<E> {
    #[inline(always)]
    fn from(remove_sink: RemoveSink) -> Self {
        Operation::RemoveSink(remove_sink)
    }
}
impl<E: Clone> From<SwapSource> for Operation<E> {
    #[inline(always)]
    fn from(swap_source: SwapSource) -> Self {
        Operation::SwapSource(swap_source)
    }
}
impl<E: Clone> From<Remover> for Operation<E> {
    #[inline(always)]
    fn from(delete: Remover) -> Self {
        Operation::RemoveExpr(delete)
    }
}
impl<E: Clone, G: Graph<Expr = E>> ModifyGraph<G> for Operation<E> {
    fn apply(&self, graph: &mut G) {
        match self {
            &Operation::Inserter(ref insert) => {
                insert.apply(graph);
            }
            &Operation::AddSink(ref add_sink) => {
                add_sink.apply(graph);
            }
            &Operation::RemoveSink(ref remove_sink) => {
                remove_sink.apply(graph);
            }
            &Operation::SwapSource(ref swap_sources) => {
                swap_sources.apply(graph);
            }
            &Operation::RemoveExpr(ref remove) => {
                remove.apply(graph);
            }
        }
    }
}
