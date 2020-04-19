use super::super::cfgbuilder::{CallStack, ExpressionCollection, HashedExpression};
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;

use super::expr::InlinedExpression;

#[derive(Default)]
pub struct InlinedCollection<'a> {
    expr: BTreeMap<u64, InlinedExpression<'a>>,
    ret: Option<u64>,
}
impl<'a> InlinedCollection<'a> {
    /// converts the ExpressionCollection into an inlined collection
    pub fn new(arg: &ExpressionCollection<'a>) -> InlinedCollection<'a> {
        let mut stack = CallStack::new(arg);
        let mut coll = InlinedCollection::default();
        let return_expr = match arg.get_return() {
            Option::None => _unreachable_panic!(),
            Option::Some(ref expr) => {
                InlinedExpression::new(expr, &mut stack, &mut coll).get_hash()
            }
        };
        ::std::mem::replace(&mut coll.ret, Some(return_expr));
        coll
    }

    /// provides the return expression value
    pub fn get_return(&self) -> Option<u64> {
        self.ret.clone()
    }

    /// returns an expression based on its hashed identifier
    pub fn get_expr<'b>(&'b self, inlined_expr: &u64) -> Option<&'b InlinedExpression<'a>> {
        self.expr.get(inlined_expr)
    }

    /// returns an iterator over the internal btreemap
    pub fn get_expression_map<'b>(&'b self) -> Iter<'b, u64, InlinedExpression<'a>> {
        self.expr.iter()
    }

    /// inserts a hashed expression, and its InlinedExpression counter part.
    pub fn insert_hash(&mut self, inlined: &InlinedExpression<'a>) {
        let inlined_hashed = inlined.get_hash();
        self.expr.insert(inlined_hashed.clone(), inlined.clone());
    }
}
