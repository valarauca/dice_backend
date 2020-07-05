use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem::replace;
use std::sync::Arc;

use super::super::inliner::{BoolArg, BoolOrInt, IntArg, Op};
use super::super::itertools::Itertools;
use super::super::smallvec::SmallVec;

use super::super::seahasher::DefaultSeaHasher;
use super::{BoolVec, Datum, Dice3, Dice6, Element, ElementVec, IntVec, Rational};

/// Iter is an iterator of elements
pub type Iter = ElementVec;

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

pub fn from_op(arg: &Op) -> Combinator {
    #[inline(always)]
    fn int_scalar<T, F>(lambda: F) -> impl 'static + Fn((Element, Element)) -> Element
    where
        F: Fn(i8, i8) -> T + 'static,
        Datum: From<T>,
    {
        move |(i1, i2): (Element, Element)| -> Element {
            let (data_1, prob_1) = i1.split();
            let (data_2, prob_2) = i2.split();
            Element::new(lambda(data_1.get_int(), data_2.get_int()), prob_1 * prob_2)
        }
    }

    #[inline(always)]
    fn int_coll_scalar<F>(lambda: F) -> impl Fn((Element, Element)) -> Element
    where
        F: Fn(&mut i8, i8),
    {
        move |(collection, item): (Element, Element)| -> Element {
            let (collection_data, collection_prob) = collection.split();
            let (scalar_data, scalar_prob) = item.split();
            let scalar_int = scalar_data.get_int();
            let mut coll_vec = collection_data.get_int_vec();
            for ptr in coll_vec.as_mut_slice().iter_mut() {
                lambda(ptr, scalar_int);
            }
            Element::new(coll_vec, scalar_prob * collection_prob)
        }
    }

    /// collection is always first arg, scalar is second
    #[inline(always)]
    fn int_coll_bool<F>(lambda: F) -> impl Fn((Element, Element)) -> Element
    where
        F: Fn(i8, i8) -> bool,
    {
        move |(collection, item): (Element, Element)| -> Element {
            let (collection_data, collection_prob) = collection.split();
            let (scalar_data, scalar_prob) = item.split();
            let scalar_int = scalar_data.get_int();
            Element::new(
                collection_data
                    .get_int_vec()
                    .into_iter()
                    .map(|x| lambda(x, scalar_int))
                    .collect::<BoolVec>(),
                scalar_prob * collection_prob,
            )
        }
    }

    match arg {
        Op::Add(IntArg::Int_Int(left, right)) => new_combin(move |i1: Iter, i2: Iter| -> Iter {
            new_iter(
                i1.into_iter()
                    .cartesian_product(small_vec_builder(i2))
                    .map(int_scalar(|a, b| a + b)),
            )
        }),
        Op::Add(IntArg::Int_CollectionOfInt(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i2.into_iter()
                        .cartesian_product(small_vec_builder(i1))
                        .map(int_coll_scalar(|a, b| *a += b)),
                )
            })
        }
        Op::Add(IntArg::CollectionOfInt_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_coll_scalar(|a, b| *a += b)),
                )
            })
        }
        Op::Sub(IntArg::Int_Int(left, right)) => new_combin(move |i1: Iter, i2: Iter| -> Iter {
            new_iter(
                i1.into_iter()
                    .cartesian_product(small_vec_builder(i2))
                    .map(int_scalar(|a, b| a - b)),
            )
        }),
        Op::Sub(IntArg::Int_CollectionOfInt(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i2.into_iter()
                        .cartesian_product(small_vec_builder(i1))
                        .map(int_coll_scalar(|a, b| *a -= b)),
                )
            })
        }
        Op::Sub(IntArg::CollectionOfInt_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_coll_scalar(|a, b| *a -= b)),
                )
            })
        }
        Op::Mul(IntArg::Int_Int(left, right)) => new_combin(move |i1: Iter, i2: Iter| -> Iter {
            new_iter(
                i1.into_iter()
                    .cartesian_product(small_vec_builder(i2))
                    .map(int_scalar(|a, b| a * b)),
            )
        }),
        Op::Mul(IntArg::Int_CollectionOfInt(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i2.into_iter()
                        .cartesian_product(small_vec_builder(i1))
                        .map(int_coll_scalar(|a, b| *a *= b)),
                )
            })
        }
        Op::Mul(IntArg::CollectionOfInt_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_coll_scalar(|a, b| *a *= b)),
                )
            })
        }
        Op::Div(IntArg::Int_Int(left, right)) => new_combin(move |i1: Iter, i2: Iter| -> Iter {
            new_iter(
                i1.into_iter()
                    .cartesian_product(small_vec_builder(i2))
                    .map(int_scalar(|a, b| a / b)),
            )
        }),
        Op::Div(IntArg::Int_CollectionOfInt(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i2.into_iter()
                        .cartesian_product(small_vec_builder(i1))
                        .map(int_coll_scalar(|a, b| *a /= b)),
                )
            })
        }
        Op::Div(IntArg::CollectionOfInt_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_coll_scalar(|a, b| *a /= b)),
                )
            })
        }
        Op::GreaterThan(IntArg::Int_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_scalar(|a, b| a > b)),
                )
            })
        }
        Op::GreaterThan(IntArg::Int_CollectionOfInt(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i2.into_iter()
                        .cartesian_product(small_vec_builder(i1))
                        .map(int_coll_bool(|coll, scal| scal > coll)),
                )
            })
        }
        Op::GreaterThan(IntArg::CollectionOfInt_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_coll_bool(|coll, scal| coll > scal)),
                )
            })
        }
        Op::GreaterThanEqual(IntArg::Int_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_scalar(|a, b| a >= b)),
                )
            })
        }
        Op::GreaterThanEqual(IntArg::Int_CollectionOfInt(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i2.into_iter()
                        .cartesian_product(small_vec_builder(i1))
                        .map(int_coll_bool(|coll, scal| scal >= coll)),
                )
            })
        }
        Op::GreaterThanEqual(IntArg::CollectionOfInt_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_coll_bool(|coll, scal| coll >= scal)),
                )
            })
        }
        Op::LessThan(IntArg::Int_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_scalar(|a, b| a < b)),
                )
            })
        }
        Op::LessThan(IntArg::Int_CollectionOfInt(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i2.into_iter()
                        .cartesian_product(small_vec_builder(i1))
                        .map(int_coll_bool(|coll, scal| scal < coll)),
                )
            })
        }
        Op::LessThan(IntArg::CollectionOfInt_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_coll_bool(|coll, scal| coll < scal)),
                )
            })
        }
        Op::LessThanEqual(IntArg::Int_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_scalar(|a, b| a <= b)),
                )
            })
        }
        Op::LessThanEqual(IntArg::Int_CollectionOfInt(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i2.into_iter()
                        .cartesian_product(small_vec_builder(i1))
                        .map(int_coll_bool(|coll, scal| scal <= coll)),
                )
            })
        }
        Op::LessThanEqual(IntArg::CollectionOfInt_Int(left, right)) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_coll_bool(|coll, scal| scal <= coll)),
                )
            })
        }
        Op::Equal(BoolOrInt::Int(IntArg::Int_Int(left, right))) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_scalar(|a, b| a == b)),
                )
            })
        }
        Op::Equal(BoolOrInt::Int(IntArg::Int_CollectionOfInt(left, right))) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i2.into_iter()
                        .cartesian_product(small_vec_builder(i1))
                        .map(int_coll_bool(|coll, scal| scal == coll)),
                )
            })
        }
        Op::Equal(BoolOrInt::Int(IntArg::CollectionOfInt_Int(left, right))) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_coll_bool(|coll, scal| scal == coll)),
                )
            })
        }
        Op::Equal(BoolOrInt::Bool(BoolArg::Bool_Bool(left, right))) => unreachable!(),
        Op::Equal(BoolOrInt::Bool(BoolArg::Bool_CollectionOfBool(left, right))) => unreachable!(),
        Op::Equal(BoolOrInt::Bool(BoolArg::CollectionOfBool_Bool(left, right))) => unreachable!(),
        Op::NotEqual(BoolOrInt::Int(IntArg::Int_Int(left, right))) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_scalar(|a, b| a != b)),
                )
            })
        }
        Op::NotEqual(BoolOrInt::Int(IntArg::Int_CollectionOfInt(left, right))) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i2.into_iter()
                        .cartesian_product(small_vec_builder(i1))
                        .map(int_coll_bool(|coll, scal| scal != coll)),
                )
            })
        }
        Op::NotEqual(BoolOrInt::Int(IntArg::CollectionOfInt_Int(left, right))) => {
            new_combin(move |i1: Iter, i2: Iter| -> Iter {
                new_iter(
                    i1.into_iter()
                        .cartesian_product(small_vec_builder(i2))
                        .map(int_coll_bool(|coll, scal| scal != coll)),
                )
            })
        }
        Op::NotEqual(BoolOrInt::Bool(BoolArg::Bool_Bool(left, right))) => unreachable!(),
        Op::NotEqual(BoolOrInt::Bool(BoolArg::Bool_CollectionOfBool(left, right))) => {
            unreachable!()
        }
        Op::NotEqual(BoolOrInt::Bool(BoolArg::CollectionOfBool_Bool(left, right))) => {
            unreachable!()
        }
        Op::And(BoolArg::Bool_Bool(left, right)) => unreachable!(),
        Op::And(BoolArg::CollectionOfBool_Bool(left, right)) => unreachable!(),
        Op::And(BoolArg::Bool_CollectionOfBool(left, right)) => unreachable!(),
        Op::Or(BoolArg::Bool_Bool(left, right)) => unreachable!(),
        Op::Or(BoolArg::CollectionOfBool_Bool(left, right)) => unreachable!(),
        Op::Or(BoolArg::Bool_CollectionOfBool(left, right)) => unreachable!(),
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
pub fn const_int(x: i8) -> Init {
    new_init(move || -> Iter {
        let v: Option<Element> = Some(Element::new(x, Rational::from_integer(1)));
        new_iter(v)
    })
}

/// standard library max
pub fn max() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.into_iter().filter_map(|e| -> Option<Element> {
            let (datum, prob) = e.split();
            let mut dice_coll = datum.get_int_vec();
            dice_coll.sort_unstable();
            if dice_coll.len() >= 1 {
                let max = dice_coll.pop().unwrap();
                Some(Element::new(max, prob))
            } else {
                None
            }
        }))
    })
}

/// standard library min
pub fn min() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.into_iter().filter_map(|e| -> Option<Element> {
            let (datum, prob) = e.split();
            let mut dice_coll = datum.get_int_vec();
            dice_coll.sort_unstable();
            if dice_coll.len() >= 1 {
                Some(Element::new(dice_coll[0], prob))
            } else {
                None
            }
        }))
    })
}
/// standard library length operator
pub fn len() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.into_iter().map(|e| -> Element {
            let (datum, prob) = e.split();
            Element::new(datum.len(), prob)
        }))
    })
}

/// stdlib count
pub fn count() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.into_iter().map(|e| -> Element {
            let (datum, prob) = e.split();
            let count = datum.get_bool_vec().iter().filter(|x| **x).count() as i8;
            Element::new(count, prob)
        }))
    })
}

/// stdlib filter
pub fn filter() -> Combinator {
    new_combin(move |i1: Iter, i2: Iter| -> Iter {
        new_iter(
            i1.into_iter()
                .zip(i2.into_iter())
                .map(|(i1, i2)| -> Element {
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
                }),
        )
    })
}

/// stdlib sum
pub fn sum() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.into_iter().map(|e| -> Element {
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
        /*
        let (i1,i2, cache) = match crate::runtime::cache::joiner::check(i1,i2) {
            Ok(output) => return output,
            Err(((i1,i2),cache)) => (i1,i2,cache)
        };
        */

        let lambda = |a: (Element, Element)| -> Element {
            let ((datum1, prob1), (datum2, prob2)) = (a.0.split(), a.1.split());
            let joined = match (datum1, datum2) {
                (Datum::CollectionOfInt(a), Datum::CollectionOfInt(b)) => {
                    let mut a = a;
                    a.extend(b);
                    a
                }
                (a, b) => {
                    _unreachable_panic!("expected 2 collections of int, found ({:?},{:?})", a, b);
                }
            };
            Element::new(joined, prob1 * prob2)
        };
        let vec_builder = |iter: Iter| -> SmallVec<[Element; 1]> {
            let mut v = SmallVec::new();
            v.extend(iter);
            v
        };
        let output = new_iter(
            i1.into_iter()
                .cartesian_product(vec_builder(i2))
                .map(lambda),
        );
        //crate::runtime::cache::joiner::insert(cache, &output);
        output
    })
}

pub fn d3() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.into_iter().flat_map(|e| {
            let (datum, prob) = e.split();
            // this will panic if the type checker failes
            roll_dice3(datum.get_int() as usize, prob)
        }))
    })
}

pub fn d6() -> Chain {
    new_chain(move |iter: Iter| -> Iter {
        new_iter(iter.into_iter().flat_map(|e| {
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
    /*
    match crate::runtime::cache::dice6::check_value((num, base_prob)) {
        Option::Some(item) => return item,
        _ => {}
    };
    */
    // lambda for the base case (rolling 1 dice)
    let lambda =
        move |x: i8| -> Element { Element::new([x], base_prob / Rational::from_integer(6)) };

    // lambda for other cases (rolling >1 dice)
    let joiner = move |tup: (Element, i8)| -> Element {
        let (e, x) = (tup.0, tup.1);
        let (mut datum, prob) = e.split();
        datum.append_int(x);
        Element::new(datum, prob / Rational::from_integer(6))
    };

    let output = match num {
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
                    .into_iter()
                    .cartesian_product(Dice6::new().into_iter())
                    .map(joiner),
            )
        }
    };
    //crate::runtime::cache::dice6::insert_value((num, base_prob), output.clone());
    output
}
/// generate a specific number of `dice3` rolles
fn roll_dice3(num: usize, base_prob: Rational) -> Iter {
    /*
    match crate::runtime::cache::dice3::check_value((num, base_prob)) {
        Option::Some(arg) => return arg,
        _ => {}
    };
    */

    // lambda for the base case (rolling 1 dice)
    let lambda =
        move |x: i8| -> Element { Element::new([x], base_prob / Rational::from_integer(3)) };

    // lambda for other cases (rolling >1 dice)
    let joiner = move |tup: (Element, i8)| -> Element {
        let (e, x) = (tup.0, tup.1);
        let (mut datum, prob) = e.split();
        datum.append_int(x);
        Element::new(datum, prob / Rational::from_integer(3))
    };

    let output = match num {
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
                    .into_iter()
                    .cartesian_product(Dice3::new().into_iter())
                    .map(joiner),
            )
        }
    };
    //crate::runtime::cache::dice3::insert_value((num, base_prob), output.clone());
    output
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
    ElementVec::new(arg)
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
    Box::new(move |i1: Iter| -> Iter {
        match crate::runtime::cache::chain::check::<F>(i1) {
            Ok(output) => output,
            Err((i1, cache)) => {
                let output = arg(i1);
                crate::runtime::cache::chain::insert::<F>(cache, &output);
                output
            }
        }
    })
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
    Box::new(move |i1: Iter, i2: Iter| -> Iter {
        let (i1, i2, cache) = match crate::runtime::cache::coalesce::check::<F>(i1, i2) {
            Ok(cached) => return cached,
            Err(((i1, i2), cache)) => (i1, i2, cache),
        };
        let output = arg(i1, i2);
        crate::runtime::cache::coalesce::insert::<F>(cache, output.clone());
        output
    })
}

#[inline(always)]
fn new_coalesce<F>(arg: F) -> Coalesce
where
    F: Fn(Iter) -> Init + 'static,
{
    Box::new(arg)
}

fn small_vec_builder(arg: Iter) -> SmallVec<[Element; 4]> {
    let mut v = SmallVec::new();
    v.extend(arg);
    v
}
