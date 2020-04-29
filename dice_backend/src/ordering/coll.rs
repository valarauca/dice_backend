use std::collections::BTreeMap;
use std::ops::Index;

use super::super::inliner::InlinedCollection;
use super::super::parser_output::TypeData;

use super::expr::OrderedExpression;
use super::ord::{OrdTrait, OrdType};

/// OrderedCollection is the read-only collection of statements
#[derive(Clone)]
pub struct OrderedCollection {
    data: BTreeMap<u64, OrderedExpression>,
    ret: u64,
}
impl OrderedCollection {
    /// build a new ordered collection
    pub fn new(old_coll: &InlinedCollection) -> OrderedCollection {
        let mut new_coll = OrderingCollection::default();

        // look up the old return statement
        let ret_id = old_coll.get_return().unwrap();
        let ret = old_coll.get_expr(&ret_id).unwrap();

        // recursively walk the AST to build sources & sinks.
        OrderedExpression::new(ret, old_coll, &mut new_coll);

        OrderedCollection {
            data: new_coll.data,
            ret: ret_id,
        }
    }

    /// returns the identifier of the return statement
    pub fn get_return(&self) -> u64 {
        self.ret.clone()
    }

    pub fn get_expr<'a>(&'a self, expr: u64) -> Option<&'a OrderedExpression> {
        self.data.get(&expr)
    }

    pub fn get_mut_expr<'a>(&'a mut self, expr: u64) -> Option<&'a mut OrderedExpression> {
        self.data.get_mut(&expr)
    }

    pub fn remove_expr(&mut self, expr: u64) {
        self.data.remove(&expr);
    }
}

/// OrderingCollection is used to build the `OrderedCollection`.
#[derive(Default, Clone)]
pub struct OrderingCollection {
    data: BTreeMap<u64, OrderedExpression>,
}
impl OrderingCollection {
    /// do we have this id in the collection?
    pub fn contains(&self, id: &u64) -> bool {
        self.get_expr(id).is_some()
    }

    /// attempt to insert something into the collection
    pub fn insert(&mut self, arg: OrderedExpression) {
        let own_id = arg.get_own_id();
        if !self.contains(&own_id) {
            self.data.insert(own_id, arg);
        }
    }

    /// return an expression by its identifier
    pub fn get_expr<'a>(&'a self, arg: &u64) -> Option<&'a OrderedExpression> {
        self.data.get(arg)
    }

    /// tell an expression that it'll be used in the future
    pub fn set_expr_sink(
        &mut self,
        expr_to_modify: &u64,
        sink_expr_id: u64,
        sink_expr_expected_type: TypeData,
    ) {
        #[allow(unused_mut)]
        let mut expr = self.get_mut_expr(expr_to_modify).unwrap();
        expr.add_sink(sink_expr_id, sink_expr_expected_type);
    }

    /// mutable lookup
    fn get_mut_expr<'a>(&'a mut self, arg: &u64) -> Option<&'a mut OrderedExpression> {
        self.data.get_mut(arg)
    }
}
