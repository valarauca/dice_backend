
use super::super::super::parser_output::{TypeData};
use super::super::super::ordering::{
    OrdTrait, OrderedCollection, OrderedExpression, StdLibraryFunc,
};

use super::super::traits::PeepholeMatcher;

use super::super::graphs::*;

/// InlineAdd attempts to inline an addition operation.
///
/// This requires 1 of the 2 arguments to the Add expression is a constant.
#[derive(Default)]
pub struct InlineAdd {
    _avoid_unsided: bool,
}
impl PeepholeMatcher<OrderedExpression> for InlineAdd {
    fn item_match(
        &self,
        expr: u64,
        coll: &OrderedCollection,
    ) -> Option<Modifications<OrderedExpression>> {
        let mut mods = Modifications::default();

        // assert we're dealing with addition
        let inner = match coll.get_expr(expr) {
            Option::Some(OrderedExpression::Op(Op::Add(ref inner))) => {
                if inner != TypeData::Int {
                    return None;
                }
                inner
            },
            _ => return None
        };

        // both arguments must be a constant

        // is argument 0 a constant?
        let (x, x_args) = match coll.get_expr(inner.get_sources()[0].0) {
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(ref x,ref x_args))) => {
                (*x, x_args)
            },
            _ => return None
        };
        // is argument 1 a constant?
        let (y, y_args) = match coll.get_expr(inner.get_sources()[1].0) {
            Option::Some(OrderedExpression::Constant(ConstantValue::Int(ref y,ref y_args))) => {
                (*y, y_args)
            },
            _ => return None
        };

        // oh cool we can patch the AST
        // fuck how do we generate ID's?!?!
        mods.push(Insert::new(OrderedExpression::Constant(ConstantValue::Int(
    }
}
