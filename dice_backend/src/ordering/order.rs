
use std::collections::BTreeMap;

use super::super::runner::{InlinedCollection, InlinedExpression};
use super::expr::*;


struct OrderingCollection<'a> {
    data: BTreeMap<u64,OrderedExpression<'a>>
}
impl<'a> OrderingCollection<'a> {

    pub fn new(arg: &InlinedCollection<'a>) -> Self {
        let mut item = Self::init(arg);
        // ensure expressions know where their sinks & sources are
        item.update_reader_coll();
        item
    }


    /// build a new collection
    fn init(arg: &InlinedCollection<'a>) -> Self {
        Self {
            data: arg
                .get_expression_map()
                .map(|(key,value)| (*key,OrderedExpression::new(&value)))
                .collect()
        }
    }

    /// collect all the `(sink,source)` pairs,
    /// marker the `source` that is has a reader in `sink`
    fn update_reader_coll(&mut self) {
        let sinksource = self.get_ordering();
        for (sink,source) in sinksource.iter() {
            self.update_reader(*sink,*source);
            // naive attempt to set ordering
            self.update_order(*sink,*source);
        }
    }

    /// let an expression know it'll be read
    fn update_reader(&mut self, sink: u64, source: u64) {
        self.get_mut(&source).add_sink(sink);
    }

    /// ensure the sink has a higher "source ordering" than
    /// then it's source
    fn update_order(&mut self, sink: u64, source: u64) {
        let sink_ordering = self.get(&sink).get_ordering();
        let source_ordering = self.get(&source).get_ordering();
        if sink_ordering <= source_ordering {
            self.get_mut(&sink).set_ordering(source_ordering + 1);
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
