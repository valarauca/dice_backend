
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

    pub fn get_expr<'b>(&'b self) -> Iter<'b, u64, OrderedExpression> {
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


/*
pub fn order_expressions<'a>(
    arg: &InlinedCollection<'a>
) {

    // build a map of (identifier, expression)
    let mut expr_map: BTreeMap<u64,OrderedExpression<'a>> = arg
        .get_expression_map()
        .map(|(key,value)| (*key, OrderedExpression::new(&value)))
        .collect();

   
    // process the map of (identifier, expression)
    // to reduce it into (identifier, requirement)
    let taking_lambda = |tup: (&u64,&OrderedExpression<'a>)| -> Vec<(u64,u64)> {
        tup.1.get_source_sink()
    };

    // build an array of (hash, requirement_hash)
    let tracking: Vec<(u64,u64)> = expr_map
        .iter()
        .flat_map(taking_lambda)
        .collect();

    for (sink,source) in tracking {
        match expr_map.get_mut(&source) {
            Option::None => {
                panic!("expression should exist, it was just inserted")
            },
            Option::Some(ref mut op) => {
            }
        }
    }
}
*/
