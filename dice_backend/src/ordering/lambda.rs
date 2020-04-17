use super::super::seahasher::DefaultSeaHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::super::itertools::Itertools;

use super::{Datum, Dice3, Dice6, Element, ElementFilter, ElementIterator};

/// Iter is an iterator of elements
pub type Iter = Box<dyn Iterator<Item = Element>>;

/*
 * Base Lambdas
 *
 * These Lambda Expressions are invoked at runtime
 *
 */

/// Filter is a lambda for converting
///   Element -> None
///   Element -> Element
///   Element -> vec![Element]
pub type Filter = Box<dyn Fn(Element) -> ElementFilter + 'static>;

/// Chain is something that operators on individual elements of an iterator
/// it smoothly transforms an iterator
pub type Chain = Box<dyn Fn(Iter) -> Iter + 'static>;

/// Init is used to initialize an chain
pub type Init = Box<dyn Fn() -> Iter + 'static>;

/// Combinator joins 2 arguments
pub type Combinator = Box<dyn Fn(Iter, Iter) -> Iter + 'static>;

/// Coalesce reifies an iterator stream so it can be restarted
pub type Coalesce = Box<dyn Fn(Iter) -> Init + 'static>;

/*
 * Generators
 *
 * Lambdas which return a lambda
 *
 */

pub type FilterGenerator = Box<dyn Fn() -> Filter + 'static>;
pub type ChainGenerator = Box<dyn Fn() -> Chain + 'static>;
pub type InitGenerator = Box<dyn Fn() -> Init + 'static>;
pub type CombinatorGenerator = Box<dyn Fn() -> Combinator + 'static>;
pub type CoalesceGenerator = Box<dyn Fn() -> Coalesce + 'static>;

/// build a constant boolean
pub fn const_bool(b: bool) -> Init {
    new_init(move || -> Iter {
        let v: Option<Element> = Some(Element::new(b, 1.0));
        new_iter(v)
    })
}

/// build a constant int generator
pub fn const_int(x: i32) -> Init {
    new_init(move || -> Iter {
        let v: Option<Element> = Some(Element::new(x, 1.0));
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

/// coalesce is used to handle arguments which need to be loaded multiple times.
pub fn coalesce() -> Coalesce {
    new_coalesce(move |iter: Iter| -> Init {
        // build a map and merge values
        let mut map = HashMap::<Datum, f64, DefaultSeaHasher>::with_capacity_and_hasher(
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
            let ((mut datum1, prob1), (datum2, prob2)) = (a.0.split(), a.1.split());
            datum1.extend_from(datum2.get_int_vec().into_iter().map(|x| *x));
            Element::new(datum1, prob1 * prob2)
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
fn roll_dice6(num: usize, base_prob: f64) -> Iter {
    // lambda for the base case (rolling 1 dice)
    let lambda = move |x: i32| -> Element { Element::new(vec![x], base_prob / 6.0) };

    // lambda for other cases (rolling >1 dice)
    let joiner = move |tup: (Element, i32)| -> Element {
        let (e, x) = (tup.0, tup.1);
        let (mut datum, prob) = e.split();
        datum.append_int(x);
        Element::new(datum, prob / 6.0)
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
                roll_dice3(num - 1, base_prob)
                    .cartesian_product(Dice3::new().into_iter())
                    .map(joiner),
            )
        }
    }
}
/// generate a specific number of `dice3` rolles
fn roll_dice3(num: usize, base_prob: f64) -> Iter {
    // lambda for the base case (rolling 1 dice)
    let lambda = move |x: i32| -> Element { Element::new(vec![x], base_prob / 3.0) };

    // lambda for other cases (rolling >1 dice)
    let joiner = move |tup: (Element, i32)| -> Element {
        let (e, x) = (tup.0, tup.1);
        let (mut datum, prob) = e.split();
        datum.append_int(x);
        Element::new(datum, prob / 3.0)
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
fn new_filter<F>(arg: F) -> Filter
where
    F: Fn(Element) -> ElementFilter + 'static,
{
    Box::new(arg)
}

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

/*
 * Generator type helper functions
 *
 */

#[inline(always)]
fn new_filter_gen<F>(arg: F) -> FilterGenerator
where
    F: Fn() -> Filter + 'static,
{
    Box::new(arg)
}

#[inline(always)]
fn new_chain_gen<F>(arg: F) -> ChainGenerator
where
    F: Fn() -> Chain + 'static,
{
    Box::new(arg)
}

#[inline(always)]
fn new_init_gen<F>(arg: F) -> InitGenerator
where
    F: Fn() -> Init + 'static,
{
    Box::new(arg)
}

#[inline(always)]
fn new_combin_gen<F>(arg: F) -> CombinatorGenerator
where
    F: Fn() -> Combinator + 'static,
{
    Box::new(arg)
}

#[inline(always)]
fn new_coalesce_gen<F>(arg: F) -> CoalesceGenerator
where
    F: Fn() -> Coalesce + 'static,
{
    Box::new(arg)
}
