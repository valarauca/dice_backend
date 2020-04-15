
use std::collections::BTreeMap;
use std::collections::btree_map::Iter;

use super::super::runner::{InlinedCollection, InlinedExpression};
use super::expr::*;

/// Ordering Collection assigns sources & sinks for expressions
pub struct OrderingCollection<'a> {
    data: BTreeMap<u64,OrderedExpression<'a>>,
    ret: u64,
}
impl<'a> OrderingCollection<'a> {

    pub fn new(arg: &InlinedCollection<'a>) -> Self {
        let mut item = Self::init(arg);
        // ensure expressions know where their sinks & sources are
        item.update_reader_coll();
        item
    }

    pub fn get_return(&self) -> u64 {
        self.ret
    }

    pub fn get_expr<'b>(&'b self, arg: &u64) -> &'b OrderedExpression<'a> {
        self.data.get(arg).unwrap()
    }

    pub fn get_exprs<'b>(&'b self) -> Iter<'b, u64, OrderedExpression<'a>> {
        self.data.iter()
    }


    /// build a new collection
    fn init(arg: &InlinedCollection<'a>) -> Self {
        Self {
            data: arg
                .get_expression_map()
                .map(|(key,value)| (*key,OrderedExpression::new(&value)))
                .collect(),
            ret: arg.get_return().unwrap(),
        }
    }

    /// collect all the `(sink,source)` pairs,
    /// marker the `source` that is has a reader in `sink`
    fn update_reader_coll(&mut self) {
        let sinksource = self.get_ordering();
        for (sink,source) in sinksource.iter() {
            self.update_reader_sink(sink,source);
        }
    }

    /// returns a list of (sink, source) tuples
    fn get_ordering(&mut self) -> Vec<(u64,u64)> {
        self
            .data
            .values_mut()
            .flat_map(OrderedExpression::get_source_sink)
            .collect()
    }

    fn update_reader_sink(&mut self, sink: &u64, source: &u64) {
        self.get_mut(source).add_sink(*sink);
    }

    /// get an expression as a mutable pointer
    fn get_mut<'b>(&'b mut self, key: &u64) -> &'b mut OrderedExpression<'a> {
        // this can't fail
        self.data.get_mut(key).unwrap()
    }

    fn get<'b>(&'b mut self, key: &u64) -> &'b OrderedExpression<'a> {
        // this can't fail
        self.data.get(key).unwrap()
    }
}


