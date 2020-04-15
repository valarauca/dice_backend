use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::marker::{Send, Sync};
use std::sync::Arc;

use super::super::parser_output::TypeData;

use super::super::rayon::iter::repeat;
use super::super::rayon::prelude::*;

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

    // for sorting internal buckets, does nothing for scalars
    fn sort_internal_safe(&mut self) {
        match &mut self.datum {
            &mut DataElement::Null | &mut DataElement::Bool(_) | &mut DataElement::Int(_) => {}
            &mut DataElement::CollofInt(ref mut v) => {
                v.sort_unstable();
            }
            &mut DataElement::CollofBool(ref mut v) => {
                v.sort_unstable();
            }
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
            &DataElement::CollofInt(ref vec) => vec.iter().sum(),
            _ => panic!("type error"),
        }
    }

    fn push_coll_int(&mut self, arg: i32) {
        match &mut self.datum {
            &mut DataElement::CollofInt(ref mut vec) => {
                vec.push(arg);
            }
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

    fn split(self) -> (DataElement, f64) {
        (self.datum, self.prob)
    }

    fn get_type(&self) -> Option<TypeData> {
        match &self.datum {
            &DataElement::Null => None,
            &DataElement::Bool(_) => Some(TypeData::Bool),
            &DataElement::Int(_) => Some(TypeData::Int),
            &DataElement::CollofInt(_) => Some(TypeData::CollectionOfInt),
            &DataElement::CollofBool(_) => Some(TypeData::CollectionOfBool),
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
    fn is_homogenous(&self) -> Option<TypeData> {
        let mut v: Option<TypeData> = None;
        for item in self.data.iter().map(|x| x.get_type()) {
            match item {
                Option::None => return None,
                Option::Some(kind) => {
                    match v {
                        Option::None => {
                            // should only happy initially
                            v = Some(kind);
                            continue;
                        }
                        Option::Some(x) => {
                            if x != kind {
                                return None;
                            }
                        }
                    };
                }
            };
        }
        return v;
    }

    /// builds output from inputs
    pub fn new<P>(x: P) -> ProbabilityDataType
    where
        P: IntoParallelIterator<Item = TupleElement>,
    {
        let output = x
            .into_par_iter()
            .filter(|arg| !arg.is_null())
            .fold(BTreeMap::<DataElement, f64>::new, tree_push)
            .reduce_with(tree_merge);
        let col = match output {
            Option::None => Vec::new(),
            Option::Some(tree) => {
                let mut vec = Vec::<TupleElement>::with_capacity(tree.len());
                for (k, v) in tree.into_iter() {
                    vec.push(TupleElement { datum: k, prob: v });
                }
                vec
            }
        };
        ProbabilityDataType {
            data: Arc::from(col.into_boxed_slice()),
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
        self.zip_map(&other, |a, b| -> Option<TupleElement> {
            if a.get_bool() {
                Some(TupleElement {
                    datum: b.datum.clone(),
                    prob: b.prob,
                })
            } else {
                None
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
        ProbabilityDataType::new(self.data.as_ref().into_par_iter().flat_map(lambda))
    }

    fn zip_map<F>(&self, other: &ProbabilityDataType, lambda: F) -> ProbabilityDataType
    where
        F: Sync + Send + Fn(&TupleElement, &TupleElement) -> Option<TupleElement>,
    {
        ProbabilityDataType::new(
            self.data
                .as_ref()
                .into_par_iter()
                .zip_eq(other.data.as_ref().into_par_iter())
                .filter_map(move |a| lambda(a.0, a.1)),
        )
    }

    fn map<F>(&self, lambda: F) -> ProbabilityDataType
    where
        F: Sync + Send + Fn(&TupleElement) -> TupleElement,
    {
        ProbabilityDataType::new(self.data.as_ref().into_par_iter().map(lambda))
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
