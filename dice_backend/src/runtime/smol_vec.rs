use std::hash::{Hash, Hasher};
use std::sync::Arc;

use super::super::smallvec::SmallVec;
use super::Element;

/// ElementVec is not a Small ElementVector,
/// it is actually rather large being ~256bytes.
///
/// This type exists to cut down on allocations,
/// by making copies of a vector "share" their
/// heap allocations.
#[derive(Clone)]
pub struct ElementVec {
    data: Arc<SmallVec<[Element; 4]>>,
}
impl ElementVec {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Element>,
    {
        let mut sm = SmallVec::<[Element; 4]>::new();
        for item in iter.into_iter() {
            sm.push(item);
        }
        Self { data: Arc::new(sm) }
    }
}
impl PartialEq for ElementVec {
    fn eq(&self, other: &ElementVec) -> bool {
        self.data.as_ref().eq(&other.data)
    }
}
impl Eq for ElementVec {}
impl Hash for ElementVec {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.data.as_ref().hash(hasher);
    }
}

pub struct ElementVecIter {
    data: Arc<SmallVec<[Element; 4]>>,
    dist: usize,
}
impl Iterator for ElementVecIter {
    type Item = Element;
    fn next(&mut self) -> Option<Self::Item> {
        if self.dist >= self.data.as_ref().len() {
            return None;
        }
        let item = self.data.as_ref()[self.dist].clone();
        self.dist += 1;
        Some(item)
    }
}

impl IntoIterator for ElementVec {
    type Item = Element;
    type IntoIter = ElementVecIter;
    fn into_iter(self) -> Self::IntoIter {
        ElementVecIter {
            data: self.data.clone(),
            dist: 0,
        }
    }
}
