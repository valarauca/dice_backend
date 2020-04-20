use std::collections::{BTreeMap, HashMap};
use std::iter::{FromIterator, IntoIterator};

use super::super::seahasher::DefaultSeaHasher;
use super::{Datum, Element, Rational};

#[derive(Clone)]
pub struct Report {
    data: HashMap<Datum, Rational, DefaultSeaHasher>,
}
impl Report {
    /// equal is used for testing, so comparisons between the input & output can be easily done
    pub fn equal(&self, dut: &[(Datum, Rational)]) -> Result<(), String> {
        for tuple in dut {
            match self.data.get(&tuple.0) {
                Option::None => {
                    return Err(format!(
                        "could not find datum:'{:?}' in collection",
                        &tuple.0
                    ))
                }
                Option::Some(rational) => {
                    if !rational.eq(&tuple.1) {
                        return Err(format!(
                            "for datum:'{:?}' expected value:'{:?}' found:'{:?}'",
                            &tuple.0, &tuple.1, rational
                        ));
                    }
                }
            };
        }
        if self.data.len() != dut.len() {
            return Err(format!(
                "internal collection contains {:?} tuples, while arg contains {:?}",
                self.data.len(),
                dut.len()
            ));
        }
        Ok(())
    }

    /// converts the report
    fn into_raw_report(&self) -> Vec<(Datum, f64)> {
        let make_floats = |(datum, rational): (&Datum, &Rational)| -> (Datum, f64) {
            let num = rational.numer().clone() as f64;
            let denom = rational.denom().clone() as f64;
            (datum.clone(), num / denom)
        };
        let mut vec: Vec<(Datum, f64)> = self.data.iter().map(make_floats).collect();
        vec.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        vec
    }

    pub fn serialize_report<I: Into<Option<usize>>>(&self, decimal: I) -> String {
        use std::fmt::Write;

        let decimal = decimal.into().into_iter().next().unwrap_or_else(|| 12usize);
        let mut s = String::with_capacity(4096);
        for (datum, prob) in self.into_raw_report() {
            write!(
                &mut s,
                " {datum}: {prob:.decimal$}\n",
                datum = datum,
                prob = prob,
                decimal = decimal
            )
            .unwrap();
        }
        s
    }
}

impl FromIterator<Element> for Report {
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let capacity = match iter.size_hint() {
            (0, Option::None) => 0,
            (x, Option::None) => x,
            (x, Option::Some(y)) => {
                if y > x {
                    y
                } else {
                    x
                }
            }
        };
        let mut map = HashMap::<Datum, Rational, DefaultSeaHasher>::with_capacity_and_hasher(
            capacity,
            DefaultSeaHasher::default(),
        );

        // deduplicate the incoming stream
        // sum any colliding elements
        for element in iter {
            let (mut datum, prob) = element.split();
            datum.sort();
            match map.get_mut(&datum) {
                Option::Some(p) => {
                    *p += prob;
                    continue;
                }
                _ => {}
            };
            map.insert(datum, prob);
        }

        Report { data: map }
    }
}
