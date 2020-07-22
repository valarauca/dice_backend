use std::mem::replace;

use super::Datum;

/// Element is a single value
#[derive(Clone, Debug)]
pub struct Element {
    datum: Datum,
    prob: f64,
    #[allow(dead_code)]
    _pad: u64,
}
#[test]
fn assert_element_size() {
    assert_eq!(::std::mem::size_of::<Element>(), 64);
}
impl Element {
    /// build a new element from a datum
    pub fn new<T>(datum: T, prob: f64) -> Element
    where
        Datum: From<T>,
    {
        Element {
            datum: Datum::from(datum),
            _pad: 0,
            prob,
        }
    }

    /// split into components
    pub fn split(self) -> (Datum, f64) {
        (self.datum, self.prob)
    }

    /// build iterator
    pub fn build_iter(self) -> Box<dyn Iterator<Item = Element>> {
        Box::new(Some(self).into_iter())
    }
}

/*
 * PartialEq, Eq, PartialOrd, Ord
 * Is implemented incorrect for Element
 * this is done for merging data which
 * needs to be placed in a variable.
 *
 */
impl ::std::hash::Hash for Element {
    #[inline(always)]
    fn hash<H: ::std::hash::Hasher>(&self, h: &mut H) {
        self.datum.hash(h);
    }
}
impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.datum.eq(&other.datum)
    }
}
impl Eq for Element {}
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        self.datum.partial_cmp(&other.datum)
    }
}
impl Ord for Element {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.datum.cmp(&other.datum)
    }
}
