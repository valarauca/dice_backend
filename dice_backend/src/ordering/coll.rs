use std::collections::BTreeMap;
use std::ops::Index;

use super::super::inliner::InlinedCollection;
use super::super::parser_output::TypeData;

use super::{Graph, Match, MatchTrait, OrdTrait, OrdType, OrderedExpression};

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
        //OrderedExpression::new(ret, old_coll, &mut new_coll);

        // set up the special expression which contains our `Final` statement.
        let return_type = new_coll.get_expr(&ret_id).unwrap().get_kind();
        let final_id = new_coll.next_free_id();
        let end = OrderedExpression::Final(OrdType::new(
            (final_id, return_type),
            Some((ret_id, return_type)),
        ));
        new_coll.insert(end);

        OrderedCollection {
            data: new_coll.data,
            ret: final_id,
        }
    }

    /// returns the identifier of the return statement
    pub fn get_return(&self) -> u64 {
        self.ret.clone()
    }

    pub fn get_expr<'a>(&'a self, expr: u64) -> Option<&'a OrderedExpression> {
        self.data.get(&expr)
    }

    fn get_mut_expr<'a>(&'a mut self, expr: u64) -> Option<&'a mut OrderedExpression> {
        self.data.get_mut(&expr)
    }

    fn remove_expr(&mut self, expr: u64) {
        self.data.remove(&expr);
    }

    pub fn keys<'a>(&'a self) -> std::collections::btree_map::Keys<'a, u64, OrderedExpression> {
        self.data.keys()
    }

    pub fn next_free_id<I>(&self, isnt: I) -> u64
    where
        I: IntoIterator<Item = u64> + Clone,
    {
        let iter = (0u64..u64::MAX).filter_map(|x: u64| -> Option<u64> {
            let bad_value = isnt
                .clone()
                .into_iter()
                .map(|isnt_val| isnt_val == x)
                .fold(false, |x, y| x | y);
            if bad_value {
                None
            } else {
                Some(x)
            }
        });
        for i in iter {
            if self.data.get(&i).is_none() {
                return i;
            }
        }
        panic!("ZOMG");
    }
}
impl Graph for OrderedCollection {
    type Expr = OrderedExpression;

    fn insert(&mut self, expr: Self::Expr) {
        match self.data.insert(expr.get_id(), expr) {
            Option::Some(_) => {
                panic!("insert cannot collide");
            }
            _ => {}
        };
    }

    fn add_sink<A, B>(&mut self, expr: &A, new_sink: &B)
    where
        A: MatchTrait,
        B: MatchTrait + Clone,
    {
        match self.get_mut_expr(expr.get_id()) {
            Option::None => {
                _unreachable_panic!();
            }
            Option::Some(arg) => {
                debug_assert_eq!(expr.get_kind(), arg.get_kind());
                arg.add_sink(new_sink);
            }
        }
    }

    fn remove_sink<A, B>(&mut self, expr: &A, new_sink: &B)
    where
        A: MatchTrait,
        B: MatchTrait,
    {
        match self.get_mut_expr(expr.get_id()) {
            Option::None => {
                _unreachable_panic!();
            }
            Option::Some(arg) => {
                debug_assert_eq!(expr.get_kind(), arg.get_kind());
                arg.remove_sink(new_sink);
            }
        }
    }

    fn compare_and_swap_source<A, B, C>(&mut self, expr: &A, old: &B, new: &C)
    where
        A: MatchTrait,
        B: MatchTrait + Clone,
        C: MatchTrait + Clone,
    {
        match self.get_mut_expr(expr.get_id()) {
            Option::None => {
                _unreachable_panic!();
            }
            Option::Some(arg) => {
                debug_assert_eq!(expr.get_kind(), arg.get_kind());
                arg.cas_source(old, new);
            }
        }
    }

    fn remove_expr<A>(&mut self, expr: &A)
    where
        A: MatchTrait,
    {
        match self.get_expr(expr.get_id()) {
            Option::None => {
                _unreachable_panic!();
            }
            Option::Some(arg) => {
                debug_assert_eq!(expr.get_kind(), arg.get_kind());
            }
        };
        self.remove_expr(expr.get_id());
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
        let own_id = arg.get_id();
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
        let m = Match::from((sink_expr_id, sink_expr_expected_type));
        expr.add_sink(&m);
    }

    /// mutable lookup
    fn get_mut_expr<'a>(&'a mut self, arg: &u64) -> Option<&'a mut OrderedExpression> {
        self.data.get_mut(arg)
    }

    fn next_free_id(&self) -> u64 {
        for i in 0..u64::MAX {
            if self.data.get(&i).is_none() {
                return i;
            }
        }
        panic!("ZOMG");
    }
}
