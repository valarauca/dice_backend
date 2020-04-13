use std::cmp::{Ord, Ordering as cmpOrdering};
use std::collections::btree_map::IterMut;
use std::collections::BTreeMap;

pub use super::expr::{OrderedExpression, Ordering, OrderingOp};

use super::super::runner::{InlinedCollection, InlinedExpression};

#[derive(Default)]
pub struct OrderingCollection<'a> {
    expr: BTreeMap<u64, OrderedExpression<'a>>,
    ret: Option<u64>,
}
impl<'a> OrderingCollection<'a> {
    /// builds a new collection
    pub fn new(arg: &InlinedCollection<'a>) -> Self {
        let mut collection = Self::default();
        collection.expr = arg
            .get_expression_map()
            .map(|(key, value)| (*key, OrderedExpression::new(&value)))
            .collect();
        collection.ret = arg.get_return().map(|x| x.clone());
        collection
    }

    fn update_order(&mut self) {
        // get  list of every expression
        let mut base = Vec::with_capacity(self.expr.len());
        for (key, _) in self.expr.iter() {
            base.push(*key);
        }

        // build a list of tuples which represent
        // (sync,source)
        let mut args = Vec::<(u64, u64)>::with_capacity(self.expr.len());
        for key in base.iter() {
            match self.expr.get(key) {
                Option::Some(&OrderedExpression::StdLibFunc(ref stdlib)) => {
                    for reference in stdlib.arg.iter() {
                        args.push((*key, *reference));
                    }
                }
                Option::Some(OrderedExpression::Operation(ref op)) => {
                    args.push((*key, op.left));
                    args.push((*key, op.right));
                }
                _ => {}
            };
        }
        // walk over our sync/sources as set them accordingly
        for (sync, source) in args.iter() {
            let source_order = self
                .expr
                .get(source)
                .map(|expr| expr.get_ordering())
                .unwrap();
            let sync_order = self.expr.get(sync).map(|expr| expr.get_ordering()).unwrap();
            match source_order.cmp(&sync_order) {
                cmpOrdering::Equal | cmpOrdering::Less => {
                    self.expr
                        .get_mut(source)
                        .unwrap()
                        .set_ordering(sync_order + 1);
                }
                _ => {}
            }
        }
    }
}

//
//fn update_key(arg: &mut BTreeMap<u64, OrderedExpression<'a>>) {
//}
