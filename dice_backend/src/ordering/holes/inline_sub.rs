use super::super::super::ordering::*;
use super::super::super::parser_output::TypeData;

use super::super::graphs::*;
use super::interior;

pub fn sub_inline(expr: u64, coll: &OrderedCollection) -> Option<Modifications<OrderedExpression>> {
    // assert we're dealing with addition
    let sub_op = match coll.get_expr(expr) {
        Option::Some(OrderedExpression::Op(Op::Add(ref sub_op))) => {
            if sub_op != TypeData::Int {
                return None;
            }
            sub_op
        }
        _ => return None,
    };
    let new_id = coll.next_free_id(None);

    // both arguments must be a constant
    match (
        coll.get_expr(sub_op.get_sources()[0].get_id()),
        coll.get_expr(sub_op.get_sources()[1].get_id()),
    ) {
        (
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(ref x, ref x_args))),
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(ref y, ref y_args))),
        ) => {
            let (new_constant, mut mods) = interior(sub_op, new_id, TypeData::Int, x_args, y_args);
            mods.push(Inserter::new(OrderedExpression::Constant(
                ConstantValue::Int(x - y, new_constant),
            )));
            Some(mods)
        }
        _ => None,
    }
}
