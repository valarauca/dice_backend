use super::super::cfgbuilder::{CallStack, ExpressionCollection, HashedExpression};
use std::collections::BTreeMap;

use super::expr::InlinedExpression;

#[derive(Default)]
pub struct InlinedCollection<'a> {
    // hashed expressed & their InlinedCounterpart
    hashed: BTreeMap<u64, u64>,
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
    pub fn get_return<'b>(&'b self) -> Option<u64> {
        self.ret.clone()
    }

    /// returns an expression based on its hashed identifier
    pub fn get_expr<'b>(&'b self, inlined_expr: &u64) -> Option<&'b InlinedExpression<'a>> {
        self.expr.get(inlined_expr)
    }

    /// inserts a hashed expression, and its InlinedExpression counter part.
    pub fn insert_hash(&mut self, hashed_expr: &u64, inlined: &InlinedExpression<'a>) {
        let inlined_hashed = inlined.get_hash();
        self.hashed
            .insert(hashed_expr.clone(), inlined_hashed.clone());
        self.expr.insert(inlined_hashed.clone(), inlined.clone());
    }

    /// returns the Hash, and _maybe_ an InlinedExpression if a representation of that
    /// value already exists.
    pub fn get_from_hashed<'b>(
        &'b self,
        hashed: &HashedExpression<'a>,
    ) -> (u64, Option<&'b InlinedExpression<'a>>) {
        let hash_value = hashed.get_hash();
        (
            hash_value,
            self.hashed
                .get(&hash_value)
                .into_iter()
                .filter_map(|inlined| self.expr.get(inlined))
                .next(),
        )
    }
}
