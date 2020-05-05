use super::super::super::ordering::*;
use super::super::super::parser_output::TypeData;

use super::super::traits::PeepholeMatcher;

use super::super::graphs::*;

/// InlineSub attempts to inline an addition operation.
///
/// This requires 1 of the 2 arguments to the Sub expression is a constant.
#[derive(Default)]
pub struct InlineSub {
    _avoid_unsided: bool,
}
impl PeepholeMatcher<OrderedExpression> for InlineSub {
    fn item_match(
        &self,
        expr: u64,
        coll: &OrderedCollection,
    ) -> Option<Modifications<OrderedExpression>> {
        let mut mods = Modifications::default();

        // assert we're dealing with addition
        let add_op = match coll.get_expr(expr) {
            Option::Some(OrderedExpression::Op(Op::Sub(ref add_op))) => {
                if add_op != TypeData::Int {
                    return None;
                }
                add_op
            }
            _ => return None,
        };

        // both arguments must be a constant

        // is argument 0 a constant?
        let (x, x_args) = match coll.get_expr(add_op.get_sources()[0].0) {
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(ref x, ref x_args))) => {
                (*x, x_args)
            }
            _ => return None,
        };
        // is argument 1 a constant?
        let (y, y_args) = match coll.get_expr(add_op.get_sources()[1].0) {
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(ref y, ref y_args))) => {
                (*y, y_args)
            }
            _ => return None,
        };

        // oh cool we can patch the Graph!!

        // create a new OrdType to insert into our
        let new_id: u64 = coll.next_free_id();
        let mut new_constant = OrdType::new(new_id, TypeData::Int, s_v![]);

        // where ever we sink the result of the add,
        // we need to sink the result of the new constant
        for sink in add_op.get_sinks() {
            mods.push(SwapSource::new(
                sink,
                add_op.get_matcher_tuple(),
                new_constant.get_matcher_tuple(),
            ));
            new_constant.add_sink(sink.0, sink.1);
        }

        /*
         * If our args are not consumed elsewhere
         * we can remove them
         *
         */
        mods.push(RemoveSink::new(x_args, add_op));
        if x_args.get_sinks().len() == 1 {
            mods.push(Remover::new(x_args));
        }
        mods.push(RemoveSink::new(y_args, add_op));
        if y_args.get_sinks().len() == 1 {
            mods.push(Remover::new(y_args));
        }

        // our last task is to insert the new node
        mods.push(Inserter::new(OrderedExpression::Constant(
            ConstantValue::Int(x + y, new_constant),
        )));
        Some(mods)
    }
}
