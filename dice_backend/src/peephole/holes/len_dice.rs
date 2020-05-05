use super::super::super::ordering::{
    OrdTrait, OrderedCollection, OrderedExpression, StdLibraryFunc,
};

use super::super::graphs::{AddSink, Match, Modifications, RemoveSink, Remover, SwapSource};

/// LenDice handles taking the `len(roll_dice6(VAR))` or `len(roll_dice3(VAR))`
/// and reducing this to just `VAR`.

pub fn len_dice(expr: u64, coll: &OrderedCollection) -> Option<Modifications<OrderedExpression>> {
    let mut mods = Modifications::default();
    match coll.get_expr(expr) {
        Option::Some(OrderedExpression::StdLib(StdLibraryFunc::Len(ref len))) => {
            let source = len.get_sources()[0].0;
            match coll.get_expr(source) {
                Option::Some(OrderedExpression::StdLib(StdLibraryFunc::D6(ref d6))) => {
                    // we are the only reader, which means our optimization is safe
                    if d6.get_sinks().len() == 1 {
                        // remove the len call
                        mods.push(Remover::new(len.get_matcher_tuple()));
                        // remove the d6 call
                        mods.push(Remover::new(d6.get_matcher_tuple()));
                        // for every place that `len` flows into
                        // we need to point to the source of `d6`.
                        let d6_source = &d6.get_sources()[0];
                        mods.extend(
                            len.get_sinks()
                                .iter()
                                .map(|sink| SwapSource::new(sink, len, d6_source)),
                        );
                        Some(mods)
                    } else {
                        None
                    }
                }
                Option::Some(OrderedExpression::StdLib(StdLibraryFunc::D3(ref d3))) => {
                    // ensure we are the only reader of the `d3` call.
                    if d3.get_sinks().len() == 1 {
                        // remove the len call
                        mods.push(Remover::new(len.get_matcher_tuple()));
                        // remove the d6 call
                        mods.push(Remover::new(d3.get_matcher_tuple()));
                        // for every place that `len` flows into
                        // we need to point to the source of `d3`.
                        let d3_source = &d3.get_sources()[0];
                        mods.extend(
                            len.get_sinks()
                                .iter()
                                .map(|sink| SwapSource::new(sink, len, d3_source)),
                        );
                        Some(mods)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}
