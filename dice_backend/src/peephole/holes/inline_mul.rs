use super::super::super::ordering::*;
use super::super::super::parser_output::TypeData;

use super::super::traits::PeepholeMatcher;

use super::super::graphs::*;
use super::interior;

pub fn mul_inline(expr: u64, coll: &OrderedCollection) -> Option<Modifications<OrderedExpression>> {
    // assert we're dealing with addition
    let mul_op = match coll.get_expr(expr) {
        Option::Some(OrderedExpression::Op(Op::Add(ref mul_op))) => {
            if mul_op != TypeData::Int {
                return None;
            }
            mul_op
        }
        _ => return None,
    };
    let new_id = coll.next_free_id();

    // both arguments must be a constant
    match (
        coll.get_expr(mul_op.get_sources()[0].0),
        coll.get_expr(mul_op.get_sources()[1].0),
    ) {
        (
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(ref x, ref x_args))),
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(ref y, ref y_args))),
        ) => {
            let (new_constant, mut mods) = interior(mul_op, new_id, TypeData::Int, x_args, y_args);
            mods.push(Inserter::new(OrderedExpression::Constant(
                ConstantValue::Int(x * y, new_constant),
            )));
            Some(mods)
        }
        _ => None,
    }
}
