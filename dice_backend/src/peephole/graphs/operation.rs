use super::{AddSink, Graph, ModifyGraph, RemoveSink, Remover, SwapSource};

/// Operation is an enum over the permitted graph transformations.
///
/// This enum exists so it is trivial to "replay" modifications to
/// a graph.
///
/// Its trait implementations also allows for the collection of its
/// arguments to be pretty simple.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Operation {
    AddSink(AddSink),
    RemoveSink(RemoveSink),
    SwapSource(SwapSource),
    RemoveExpr(Remover),
}
impl From<AddSink> for Operation {
    #[inline(always)]
    fn from(add_sink: AddSink) -> Operation {
        Operation::AddSink(add_sink)
    }
}
impl From<RemoveSink> for Operation {
    #[inline(always)]
    fn from(remove_sink: RemoveSink) -> Operation {
        Operation::RemoveSink(remove_sink)
    }
}
impl From<SwapSource> for Operation {
    #[inline(always)]
    fn from(swap_source: SwapSource) -> Operation {
        Operation::SwapSource(swap_source)
    }
}
impl From<Remover> for Operation {
    #[inline(always)]
    fn from(delete: Remover) -> Operation {
        Operation::RemoveExpr(delete)
    }
}
impl ModifyGraph for Operation {
    fn apply<G>(&self, graph: &mut G)
    where
        G: Graph,
    {
        match self {
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
