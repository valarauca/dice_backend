use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::marker::{Send, Sync};
use std::sync::Arc;

use super::super::rayon::iter::repeat;
use super::super::rayon::prelude::*;

/// There are only 2 scalar types within our execution environment
/// booleans & integers.
///
/// This structure lets us nicely store these within `8`bytes on x64.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Data {
    b: u8,
    i: i32,
}
impl From<i32> for Data {
    fn from(x: i32) -> Data {
        Data { b: 3, i: x }
    }
}
impl From<bool> for Data {
    fn from(b: bool) -> Data {
        Data {
            b: if b { 1 } else { 0 },
            i: 0,
        }
    }
}
impl Data {
    /// return the boolean bit
    pub fn get_bool(&self) -> Option<bool> {
        match self.b {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    /// return the integer bit
    pub fn get_int(&self) -> Option<i32> {
        match self.b {
            3 => Some(self.i),
            _ => None,
        }
    }
}

/// Tuple Element contains _a_ possible outcome
/// as well as its likelihood of occuring
#[derive(Clone)]
pub struct TupleElement {
    pub datum: Vec<Data>,
    pub prob: f64,
}
impl TupleElement {
    /// create a constant integer
    pub fn constant_int(x: i32) -> TupleElement {
        TupleElement {
            datum: vec![Data::from(x)],
            prob: 1.0,
        }
    }

    /// create a constant boolean
    pub fn constant_bool(x: bool) -> TupleElement {
        TupleElement {
            datum: vec![Data::from(x)],
            prob: 1.0,
        }
    }

    pub fn sum_across(&self) -> i32 {
        self.datum
            .iter()
            .flat_map(|x| {
                x.get_int()
                    .into_iter()
                    .chain(x.get_bool().map(|b| if b { 1 } else { 0 }))
            })
            .sum()
    }

    pub fn get_first_index_as_bool(&self) -> bool {
        if self.datum.len() == 0 {
            return false;
        }
        self.datum[0]
            .get_bool()
            .into_iter()
            .chain(self.datum[0].get_int().map(|x| x != 0))
            .next()
            .unwrap_or_else(|| false)
    }
}
unsafe impl Send for TupleElement {}
unsafe impl Sync for TupleElement {}

/// ProbabilityDataType represents the what we can eventually return
#[derive(Clone)]
pub struct ProbabilityDataType {
    pub data: Arc<[TupleElement]>,
}
impl ProbabilityDataType {
    /// builds output from inputs
    pub fn new<I>(x: I) -> ProbabilityDataType
    where
        I: Iterator<Item = TupleElement>,
    {
        let size_hint = match x.size_hint() {
            (lb, Option::Some(ub)) => {
                if ub >= lb {
                    ub
                } else {
                    lb
                }
            }
            (lb, Option::None) => lb,
        };
        let mut v = Vec::with_capacity(size_hint);
        let mut x = x;
        for item in x {
            v.push(item);
        }
        ProbabilityDataType {
            data: Arc::from(v.into_boxed_slice()),
        }
    }

    pub fn rolld3(&self) -> ProbabilityDataType {
        const DICE_3: &'static [i32] = &[1, 2, 3, 4, 5, 6];
        fn build(depth: i32, arg: &mut TupleElement, coll: &mut Vec<TupleElement>) {
            if depth == 0 {
                coll.push(arg.clone());
            } else {
                for i in DICE_3 {
                    let mut item = arg.clone();
                    arg.datum.push(Data::from(*i));
                    arg.prob *= (1.0f64 / 3.0f64);
                    build(depth - 1, &mut item, coll);
                }
            }
        }
        self.expand(move |arg| -> Vec<TupleElement> {
            let mut v = Vec::new();
            if arg.datum.len() != 1 {
                return v;
            }
            let count = match arg.datum[0].get_int() {
                Option::Some(count) => count,
                Option::None => return v,
            };
            if count < 1 {
                return v;
            }
            let mut default = TupleElement {
                datum: Vec::with_capacity(count as usize),
                prob: arg.prob.clone(),
            };
            build(count as i32, &mut default, &mut v);
            v
        })
    }

    pub fn rolld6(&self) -> ProbabilityDataType {
        const DICE_6: &'static [i32] = &[1, 2, 3, 4, 5, 6];

        fn build(depth: i32, arg: &mut TupleElement, coll: &mut Vec<TupleElement>) {
            if depth == 0 {
                coll.push(arg.clone());
            } else {
                for i in DICE_6 {
                    let mut item = arg.clone();
                    arg.datum.push(Data::from(*i));
                    arg.prob *= (1.0f64 / 6.0f64);
                    build(depth - 1, &mut item, coll);
                }
            }
        }

        self.expand(move |arg| -> Vec<TupleElement> {
            let mut v = Vec::new();
            if arg.datum.len() != 1 {
                return v;
            }
            let count = match arg.datum[0].get_int() {
                Option::Some(count) => count,
                Option::None => return v,
            };
            if count < 1 {
                return v;
            }
            let mut default = TupleElement {
                datum: Vec::with_capacity(count as usize),
                prob: arg.prob.clone(),
            };
            build(count as i32, &mut default, &mut v);
            v
        })
    }

    pub fn filter(&self, other: ProbabilityDataType) -> ProbabilityDataType {
        self.zip_map(&other, |a, b| -> TupleElement {
            if a.get_first_index_as_bool() {
                TupleElement {
                    datum: b.datum.clone(),
                    prob: b.prob,
                }
            } else {
                TupleElement {
                    datum: Vec::new(),
                    prob: b.prob,
                }
            }
        })
    }

    /// sum will walk over each element within the data and resolve
    /// `datum` level sum.
    pub fn sum(&self) -> ProbabilityDataType {
        self.map(|arg: &TupleElement| -> TupleElement {
            let value: i32 = arg.sum_across();
            TupleElement {
                datum: vec![Data::from(value)],
                prob: arg.prob,
            }
        })
    }

    fn expand<F, I>(&self, lambda: F) -> ProbabilityDataType
    where
        F: Sync + Send + Fn(&TupleElement) -> I,
        I: IntoParallelIterator<Item = TupleElement>,
    {
        let v: Vec<TupleElement> = self
            .data
            .as_ref()
            .into_par_iter()
            .flat_map(lambda)
            .collect();
        ProbabilityDataType::from(v)
    }

    fn zip_map<F>(&self, other: &ProbabilityDataType, lambda: F) -> ProbabilityDataType
    where
        F: Sync + Send + Fn(&TupleElement, &TupleElement) -> TupleElement,
    {
        let v: Vec<TupleElement> = self
            .data
            .as_ref()
            .into_par_iter()
            .zip(other.data.as_ref().into_par_iter())
            .map(move |a| lambda(a.0, a.1))
            .collect();
        ProbabilityDataType::from(v)
    }

    fn map<F>(&self, lambda: F) -> ProbabilityDataType
    where
        F: Sync + Send + Fn(&TupleElement) -> TupleElement,
    {
        let v: Vec<TupleElement> = self.data.as_ref().into_par_iter().map(lambda).collect();
        ProbabilityDataType::from(v)
    }
}
impl From<Vec<TupleElement>> for ProbabilityDataType {
    fn from(v: Vec<TupleElement>) -> ProbabilityDataType {
        ProbabilityDataType {
            data: Arc::from(v.into_boxed_slice()),
        }
    }
}
unsafe impl Send for ProbabilityDataType {}
unsafe impl Sync for ProbabilityDataType {}
