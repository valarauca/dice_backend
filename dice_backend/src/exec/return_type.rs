use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::marker::{Send, Sync};
use std::sync::Arc;

use super::super::rayon::iter::repeat;
use super::super::rayon::prelude::*;

#[derive(Clone)]
pub enum DataElement {
    Null,
    Bool(bool),
    Int(i32),
    CollofInt(Vec<i32>),
    CollofBool(Vec<i32>),
}

/// Tuple Element contains _a_ possible outcome
/// as well as its likelihood of occuring
#[derive(Clone)]
pub struct TupleElement {
    pub datum: DataElement,
    pub prob: f64,
}
impl TupleElement {
    /// create a constant integer
    pub fn constant_int(x: i32) -> TupleElement {
        TupleElement {
            datum: DataElement::Int(x),
            prob: 1.0,
        }
    }

    /// create a constant boolean
    pub fn constant_bool(x: bool) -> TupleElement {
        TupleElement {
            datum: DataElement::Bool(x),
            prob: 1.0,
        }
    }

    fn is_null(&self) -> bool {
        match &self.datum {
            &DataElement::Null => true,
            _ => false,
        }
    }

    fn sum_across(&self) -> i32 {
        match &self.datum {
            &DataElement::CollofInt(ref vec) => {
                vec.iter().sum()
            },
            _ => panic!("type error"),
        }
    }

    fn push_coll_int(&mut self, arg: i32) {
        match &mut self.datum {
            &mut DataElement::CollofInt(ref mut vec) => {
                vec.push(arg);
            },
            _ => panic!("type error"),
        }
    }

    fn get_int(&self) -> i32 {
        match &self.datum {
            &DataElement::Int(x) => x,
            _ => panic!("type error"),
        }
    }

    fn get_bool(&self) -> bool {
        match &self.datum {
            &DataElement::Bool(b) => b,
            _ => panic!("type error"),
        }
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
                    arg.push_coll_int(i.clone());
                    arg.prob *= (1.0f64 / 3.0f64);
                    build(depth - 1, &mut item, coll);
                }
            }
        }
        self.expand(move |arg| -> Vec<TupleElement> {
            let mut v = Vec::new();
            let count = arg.get_int();
            if count <= 0 {
                return v;
            }
            let mut default = TupleElement {
                datum: DataElement::CollofInt(Vec::with_capacity(count as usize)),
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
                    arg.push_coll_int(i.clone());
                    arg.prob *= (1.0f64 / 6.0f64);
                    build(depth - 1, &mut item, coll);
                }
            }
        }

        self.expand(move |arg| -> Vec<TupleElement> {
            let mut v = Vec::new();
            let count = arg.get_int();
            if count <= 0 {
                return v;
            }
            let mut default = TupleElement {
                datum: DataElement::CollofInt(Vec::with_capacity(count as usize)),
                prob: arg.prob.clone(),
            };
            build(count as i32, &mut default, &mut v);
            v
        })
    }

    pub fn filter(&self, other: ProbabilityDataType) -> ProbabilityDataType {
        self.zip_map(&other, |a, b| -> TupleElement {
            if a.get_bool() {
                TupleElement {
                    datum: b.datum.clone(),
                    prob: b.prob,
                }
            } else {
                TupleElement {
                    datum: DataElement::Null,
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
                datum: DataElement::Int(value),
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
            .filter(move |x| !x.is_null())
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
