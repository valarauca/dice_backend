use super::super::cfgbuilder::{ExpressionCollection, HashedExpression};
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
    pub fn insert_hash(&mut self, hashed_expr: &u64, inlined: &InlinedExpression<'a>) {
        let inlined_hashed = inlined.get_hash();
        self.hashed
            .insert(hashed_expr.clone(), inlined_hashed.clone());
        self.expr.insert(inlined_hashed.clone(), inlined.clone());
    }
    ///
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
