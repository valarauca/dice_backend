use super::super::super::ordering::*;
use super::super::graphs::*;

/// LenDice handles taking the `len(roll_dice6(VAR))` or `len(roll_dice3(VAR))`
/// and reducing this to just `VAR`.

pub fn len_dice(expr: u64, coll: &OrderedCollection) -> Option<Modifications<OrderedExpression>> {
    // are we dealing with a length?
    let len_op = match coll.get_expr(expr).unwrap() {
        OrderedExpression::StdLib(StdLibraryFunc::Len(ref len_op)) => len_op,
        _ => return None,
    };

    // ensure we have something like `len(roll_d6(_))` or `len(roll_d3(_))`
    let roll_op = match coll.get_expr(len_op.get_sources()[0].get_id()).unwrap() {
        OrderedExpression::StdLib(StdLibraryFunc::D6(ref roll)) => roll,
        OrderedExpression::StdLib(StdLibraryFunc::D3(ref roll)) => roll,
        _ => return None,
    };

    // ensure the `roll_d6(_)` or `roll_d3(_)` point to a constant
    let count_args = match coll.get_expr(roll_op.get_sources()[0].get_id()).unwrap() {
        OrderedExpression::Constant(ConstantValue::Int(_, ref count_args)) => count_args,
        _ => return None,
    };

    let mut mods = Modifications::default();
    mods.push(SwapSource::new(Match::default(), len_op, count_args));
    // where ever `len_op` flowed into, we can replace it with the constant
    for sink in len_op.get_sinks() {
        mods.push(SwapSource::new(*sink, len_op, count_args));
        mods.push(AddSink::new(count_args, *sink));
    }
    // special case for return value

    // we can remove `len_op`
    mods.push(Remover::new(len_op));

    // inform the `roll_op` it isn't be consumed by `len_op`
    mods.push(RemoveSink::new(roll_op, len_op));
    if roll_op.get_sinks().len() == 1 {
        // if `roll_op` is only consumed once... and we just
        // removed the only consumption, we can remove it.
        mods.push(Remover::new(roll_op));
    }

    Some(mods)
}

/*
pub fn len_dice(expr: u64, coll: &OrderedCollection) -> Option<Modifications<OrderedExpression>> {
    let mut mods = Modifications::default();
    match coll.get_expr(expr) {
        Option::Some(OrderedExpression::StdLib(StdLibraryFunc::Len(ref len))) => {
            let source = len.get_sources()[0].get_id();
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
*/
