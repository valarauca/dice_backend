use super::Datum;
use std::mem::replace;

/// Element is a single value
#[derive(Clone, Debug)]
pub struct Element {
    datum: Datum,
    prob: f64,
}
impl Element {
    /// build a new element from a datum
    pub fn new<T>(datum: T, prob: f64) -> Element
    where
        Datum: From<T>,
    {
        Element {
            datum: Datum::from(datum),
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

/// ElementFilters are used for partial function application
/// it is closely tied to the element type
pub enum ElementFilter {
    None,
    Some(Element),
    Multi(Vec<Element>),
}
impl ElementFilter {
    /// optionals futfill `IntoIterator`
    pub fn new<T: IntoIterator<Item = Element>>(item: T) -> Self {
        <Self as ::std::iter::FromIterator<Element>>::from_iter(item)
    }

    pub fn build_iter(self) -> Box<dyn Iterator<Item = Element>> {
        Box::new(<Self as ::std::iter::IntoIterator>::into_iter(self))
    }
}
impl ::std::iter::FromIterator<Element> for ElementFilter {
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        // unroll the first loop
        let first = match iter.next() {
            Option::None => return Self::None,
            Option::Some(x) => x,
        };
        let second = match iter.next() {
            Option::None => return Self::Some(first),
            Option::Some(x) => x,
        };
        let mut v = vec![first, second];
        v.extend(iter);
        Self::Multi(v)
    }
}
impl IntoIterator for ElementFilter {
    type Item = Element;
    type IntoIter = ElementIterator;
    fn into_iter(self) -> ElementIterator {
        ElementIterator { item: self }
    }
}

/// Iterator over the output of a filter
pub struct ElementIterator {
    item: ElementFilter,
}
impl Iterator for ElementIterator {
    type Item = Element;
    fn next(&mut self) -> Option<Element> {
        // trivial cases return immediately
        let mut vec = match replace(&mut self.item, ElementFilter::None) {
            ElementFilter::None => return Option::None,
            ElementFilter::Some(elem) => return Option::Some(elem),
            ElementFilter::Multi(vec) => vec,
        };
        // stack pop'ing stuff
        let output = vec.pop();
        if vec.len() != 0 {
            replace(&mut self.item, ElementFilter::Multi(vec));
        }
        output
    }
}
