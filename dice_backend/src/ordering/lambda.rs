use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem::replace;

use super::super::itertools::Itertools;

use super::super::seahasher::DefaultSeaHasher;
use super::{BoolVec, Datum, Dice3, Dice6, Element, IntVec, Rational};

/// Iter is an iterator of elements
pub type Iter = Box<dyn Iterator<Item = Element>>;

/*
 * Base Lambdas
 *
 * These Lambda Expressions are invoked at runtime
 *
 */

/// Chain is something that operators on individual elements of an iterator
/// it smoothly transforms an iterator
pub type Chain = Box<dyn Fn(Iter) -> Iter + 'static>;

/// Allows a chain to be restarted
pub type CoalesceChain = Box<dyn Fn(Iter) -> Init + 'static>;

/// Init is used to initialize an chain
pub type Init = Box<dyn Fn() -> Iter + 'static>;

/// Combinator joins 2 arguments
pub type Combinator = Box<dyn Fn(Iter, Iter) -> Iter + 'static>;

/// Joins 2 iterators together, but returns a lambda which can
/// be invoked multiple times
pub type CoalesceCombinator = Box<dyn Fn(Iter, Iter) -> Init + 'static>;

/// Coalesce reifies an iterator stream so it can be restarted
pub type Coalesce = Box<dyn Fn(Iter) -> Init + 'static>;

/// LambdaKind is used for building & resolving lambdas
pub enum LambdaKind {
    None,
    Chain(Chain),
    CoalesceChain(CoalesceChain),
    Init(Init),
    Combinator(Combinator),
    CoalesceCombinator(CoalesceCombinator),
}
impl LambdaKind {
    pub fn is_idempotent(&self) -> bool {
        match self {
            &LambdaKind::None => false,
            &LambdaKind::Chain(_) => false,
            &LambdaKind::CoalesceChain(_) => true,
            &LambdaKind::Init(_) => true,
            &LambdaKind::Combinator(_) => false,
            &LambdaKind::CoalesceCombinator(_) => true,
        }
    }

    pub fn make_idempotent(&mut self) {
        if self.is_idempotent() {
            return;
        }
        let new_value = match replace(self, LambdaKind::None) {
            LambdaKind::Chain(chain) => LambdaKind::CoalesceChain(chain_to_coalesce(chain)),
            LambdaKind::Combinator(combin) => {
                LambdaKind::CoalesceCombinator(combinator_to_coalesce(combin))
            }
            x => x,
        };
        replace(self, new_value);
    }

    pub fn get_iter(&mut self, stack: &mut Vec<Iter>) -> Iter {
        match replace(self, LambdaKind::None) {
            LambdaKind::None => {
                _unreachable_panic!("invalidated b/c not idempotent");
            }
            LambdaKind::Chain(lambda) => {
                // return the iterator & invalid self
                lambda(stack.pop().unwrap())
            }
            LambdaKind::Combinator(lambda) => {
                // return the iterator & invalid self
                lambda(stack.pop().unwrap(), stack.pop().unwrap())
            }
            LambdaKind::CoalesceCombinator(lambda) => {
                // create function with can build multiple
                // copies of this iterator
                let init_func = lambda(stack.pop().unwrap(), stack.pop().unwrap());
                let iter_out = init_func();
                // update self with idemponent
                // lambda
                replace(self, LambdaKind::Init(init_func));
                // return iterator
                iter_out
            }
            LambdaKind::CoalesceChain(lambda) => {
                let init_func = lambda(stack.pop().unwrap());
                let iter_out = init_func();
                replace(self, LambdaKind::Init(init_func));
                iter_out
            }
            LambdaKind::Init(lambda) => {
                let iter_out = lambda();
                replace(self, LambdaKind::Init(lambda));
                iter_out
            }
        }
    }
}

/// build a constant boolean
pub fn const_bool(b: bool) -> Init {
    new_init(move || -> Iter {
        let v: Option<Element> = Some(Element::new(b, Rational::from_integer(1)));
        new_iter(v)
    })
}

/// build a constant int generator
pub fn const_int(x: i32) -> Init {
    new_init(move || -> Iter {
        let v: Option<Element> = Some(Element::new(x, Rational::from_integer(1)));
        new_iter(v)
    })
}

/// standard library length operator
pub fn len() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.map(|e| -> Element {
            let (datum, prob) = e.split();
            Element::new(datum.len() as i32, prob)
        }))
    })
}

/// stdlib count
pub fn count() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.map(|e| -> Element {
            let (datum, prob) = e.split();
            let count = datum.get_bool_vec().iter().filter(|x| **x).count() as i32;
            Element::new(count, prob)
        }))
    })
}

/// stdlib filter
pub fn filter() -> Combinator {
    new_combin(move |i1: Iter, i2: Iter| -> Iter {
        new_iter(i1.zip(i2).map(|(i1, i2)| -> Element {
            let (d1, p1) = i1.split();
            let (d2, p2) = i2.split();
            // their source should be identical
            assert_eq!(p1, p2);
            let v: IntVec = d1
                .get_bool_vec()
                .into_iter()
                .zip(d2.get_int_vec())
                .filter_map(|(b, i)| if b { Some(i) } else { None })
                .collect();
            Element::new(v, p1)
        }))
    })
}

/// stdlib sum
pub fn sum() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.map(|e| -> Element {
            let (datum, prob) = e.split();
            Element::new(datum.sum(), prob)
        }))
    })
}

/// coalesce is used to handle arguments which need to be loaded multiple times.
pub fn coalesce() -> Coalesce {
    new_coalesce(move |iter: Iter| -> Init {
        // build a map and merge values
        let mut map = HashMap::<Datum, Rational, DefaultSeaHasher>::with_capacity_and_hasher(
            100,
            DefaultSeaHasher::default(),
        );
        for element in iter {
            // sort so similiar combinations match
            let (mut datum, prob) = element.split();
            datum.sort();
            // if the same item exists sum probability
            match map.get_mut(&datum) {
                Option::Some(p) => {
                    *p += prob;
                    continue;
                }
                _ => {}
            };
            map.insert(datum, prob);
        }
        // build the init lambda
        new_init(move || -> Iter {
            // return a clone of map every time init is invoked
            new_iter(map.clone().into_iter().map(|(k, v)| Element::new(k, v)))
        })
    })
}

/// Method of joining 2 iterator streams
pub fn join() -> Combinator {
    new_combin(move |i1: Iter, i2: Iter| -> Iter {
        let lambda = |a: (Element, Element)| -> Element {
            let ((datum1, prob1), (datum2, prob2)) = (a.0.split(), a.1.split());
            let joined = match (datum1,datum2) {
                (Datum::CollectionOfInt(a),Datum::CollectionOfInt(b)) => {
                    let mut a = a;
                    a.extend(b);
                    a
                },
                (a,b) => {
                    _unreachable_panic!("expected 2 collections of int, found ({:?},{:?})", a,b);
                }
            };
            Element::new(joined, prob1 * prob2)
        };
        let vec_builder = |iter: Iter| -> Vec<Element> {
            let mut v = Vec::new();
            v.extend(iter);
            v
        };
        new_iter(i1.cartesian_product(vec_builder(i2)).map(lambda))
    })
}

pub fn d3() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.flat_map(|e| {
            let (datum, prob) = e.split();
            // this will panic if the type checker failes
            roll_dice3(datum.get_int() as usize, prob)
        }))
    })
}

pub fn d6() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.flat_map(|e| {
            let (datum, prob) = e.split();
            // this will panic if the type checker failes
            roll_dice6(datum.get_int() as usize, prob)
        }))
    })
}

/*
 * Private Dice Roller Functions
 *
 */

/// generate a specific number of `dice3` rolles
fn roll_dice6(num: usize, base_prob: Rational) -> Iter {
    // lambda for the base case (rolling 1 dice)
    let lambda =
        move |x: i32| -> Element { Element::new([x], base_prob / Rational::from_integer(6)) };

    // lambda for other cases (rolling >1 dice)
    let joiner = move |tup: (Element, i32)| -> Element {
        let (e, x) = (tup.0, tup.1);
        let (mut datum, prob) = e.split();
        datum.append_int(x);
        Element::new(datum, prob / Rational::from_integer(6))
    };

    match num {
        0 => {
            // zero need to avoid recursion
            new_iter(None)
        }
        1 => {
            // this is the _normal_ base case
            // recursion should catch 1 before other cases
            // so it can iterate correctly
            new_iter(Dice6::new().into_iter().map(lambda))
        }
        _ => {
            // do recursive stuff for values > 2
            new_iter(
                roll_dice6(num - 1, base_prob)
                    .cartesian_product(Dice6::new().into_iter())
                    .map(joiner),
            )
        }
    }
}
/// generate a specific number of `dice3` rolles
fn roll_dice3(num: usize, base_prob: Rational) -> Iter {
    // lambda for the base case (rolling 1 dice)
    let lambda =
        move |x: i32| -> Element { Element::new([x], base_prob / Rational::from_integer(3)) };

    // lambda for other cases (rolling >1 dice)
    let joiner = move |tup: (Element, i32)| -> Element {
        let (e, x) = (tup.0, tup.1);
        let (mut datum, prob) = e.split();
        datum.append_int(x);
        Element::new(datum, prob / Rational::from_integer(3))
    };

    match num {
        0 => {
            // zero need to avoid recursion
            new_iter(None)
        }
        1 => {
            // this is the _normal_ base case
            // recursion should catch 1 before other cases
            // so it can iterate correctly
            new_iter(Dice3::new().into_iter().map(lambda))
        }
        _ => {
            // do recursive stuff for values > 2
            new_iter(
                roll_dice3(num - 1, base_prob)
                    .cartesian_product(Dice3::new().into_iter())
                    .map(joiner),
            )
        }
    }
}

/*
 * Converstion Types
 *
 */

#[inline(always)]
pub fn chain_to_coalesce(chain_arg: Chain) -> CoalesceChain {
    Box::new(move |iter: Iter| -> Init {
        let lambda = coalesce();
        lambda(chain_arg(iter))
    })
}

#[inline(always)]
pub fn combinator_to_coalesce(arg: Combinator) -> CoalesceCombinator {
    Box::new(move |i1: Iter, i2: Iter| -> Init {
        let lambda = coalesce();
        lambda(arg(i1, i2))
    })
}

/*
 * Iterator builder
 *
 */

#[inline(always)]
fn new_iter<I>(arg: I) -> Iter
where
    I: IntoIterator<Item = Element> + 'static,
{
    Box::new(arg.into_iter())
}

/*
 * Base type helper functions
 *
 */

#[inline(always)]
fn new_chain<F>(arg: F) -> Chain
where
    F: Fn(Iter) -> Iter + 'static,
{
    Box::new(arg)
}

#[inline(always)]
fn new_init<F>(arg: F) -> Init
where
    F: Fn() -> Iter + 'static,
{
    Box::new(arg)
}

#[inline(always)]
fn new_combin<F>(arg: F) -> Combinator
where
    F: Fn(Iter, Iter) -> Iter + 'static,
{
    Box::new(arg)
}

#[inline(always)]
fn new_coalesce<F>(arg: F) -> Coalesce
where
    F: Fn(Iter) -> Init + 'static,
{
    Box::new(arg)
}
