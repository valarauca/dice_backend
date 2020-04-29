use super::graphs::Modifications;

use super::super::ordering::OrderedCollection;

/// PeepholeMatcher is used to match expressions
pub trait PeepholeMatcher {
    /// return a list of modifications, if the peeplehole is
    /// detected.
    fn item_match(&self, expr: u64, coll: &OrderedCollection) -> Option<Modifications>;
}
