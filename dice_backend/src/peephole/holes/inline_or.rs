use super::super::super::ordering::*;
use super::super::super::parser_output::TypeData;

use super::super::traits::PeepholeMatcher;

use super::super::graphs::*;

use super::interior;

pub fn or_inline(expr: u64, coll: &OrderedCollection) -> Option<Modifications<OrderedExpression>> {
    // assert we're dealing with addition
    let and_or = match coll.get_expr(expr) {
        Option::Some(OrderedExpression::Op(Op::And(ref and_or))) => {
            if and_or != TypeData::Int {
                return None;
            }
            and_or
        }
        _ => return None,
    };

    let new_id = coll.next_free_id();

    // both arguments must be a constant
    match (
        coll.get_expr(and_or.get_sources()[0].0),
        coll.get_expr(and_or.get_sources()[1].0),
    ) {
        (
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(ref x, ref x_args))),
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(ref y, ref y_args))),
        ) => {
            let (new_constant, mut mods) = interior(and_or, new_id, TypeData::Int, x_args, y_args);
            mods.push(Inserter::new(OrderedExpression::Constant(
                ConstantValue::Int(*x & *y, new_constant),
            )));
            Some(mods)
        }
        (
            Option::Some(OrderedExpression::Constant(ConstantValue::Bool(ref x, ref x_args))),
            Option::Some(OrderedExpression::Constant(ConstantValue::Bool(ref y, ref y_args))),
        ) => {
            let (new_constant, mut mods) = interior(and_or, new_id, TypeData::Bool, x_args, y_args);
            mods.push(Inserter::new(OrderedExpression::Constant(
                ConstantValue::Bool(*x & *y, new_constant),
            )));
            Some(mods)
        }
        _ => None,
    }
}
