use super::super::smallvec::SmallVec;

use super::super::parser_output::TypeData;
use super::matcher::{Match, MatchTrait};

/// A collection of expressions that -this- expression requires
/// as arguments, or the expressions which require -this- as
/// an argument.
pub type ExprVec = SmallVec<[Match; 2]>;

/// OrdTrait contains information about the ordering of an expression
pub trait OrdTrait: MatchTrait + AsRef<OrdType> + AsMut<OrdType> {
    /// return a match item
    fn get_matcher(&self) -> Match {
        let item: &Match = self.as_ref();
        item.clone()
    }

    /// returns what this expression must read
    fn get_sources<'a>(&'a self) -> &'a [Match] {
        let item: &'a OrdType = self.as_ref();
        item.sources.as_slice()
    }

    /// are there any sources?
    fn has_sources(&self) -> bool {
        self.get_sources().len() > 1
    }

    /// returns where this expression results;
    fn get_sinks<'a>(&'a self) -> &'a [Match] {
        let item: &'a OrdType = self.as_ref();
        item.sinks.as_slice()
    }

    /// are their any sinks?
    fn has_sinks(&self) -> bool {
        self.get_sinks().len() > 1
    }

    /*
     * Types that mutate
     * (requires for construction)
     *
     */

    fn add_sink<M>(&mut self, matcher: &M)
    where
        M: MatchTrait + Sized,
    {
        // this must be true
        let self_type = self.get_kind();
        let m_type = self.get_kind();
        debug_assert!(self_type == m_type);

        let item: &mut OrdType = self.as_mut();

        if !item.sink_exists(matcher) {
            item.sinks.push(matcher.as_ref().clone());
        }
    }

    fn cas_source<A, B>(&mut self, old: &A, new: &B)
    where
        A: MatchTrait + Sized,
        B: MatchTrait + Clone,
    {
        match search_for_index(self.get_sources(), old) {
            Option::None => {}
            Option::Some(index) => {
                self.as_mut().sources[index] = new.as_ref().clone();
            }
        };
    }

    fn remove_sink<M>(&mut self, matcher: &M)
    where
        M: MatchTrait,
    {
        match search_for_index(self.get_sinks(), matcher) {
            Option::None => {
                // do nothing
            }
            Option::Some(index) => {
                self.as_mut().sinks.remove(index);
            }
        };
    }
}

#[inline(always)]
fn search_for_index<A, B>(slice: &[A], arg: &B) -> Option<usize>
where
    A: MatchTrait,
    B: MatchTrait,
{
    slice
        .iter()
        .enumerate()
        .filter_map(|(index, item)| -> Option<usize> {
            if item.as_ref().eq(arg.as_ref()) {
                Some(index)
            } else {
                None
            }
        })
        .next()
}

/// OrdType encodes information about ordering.
#[derive(Default, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrdType {
    id: Match,
    sources: ExprVec,
    sinks: ExprVec,
}
impl OrdType {
    /// build a new `OrdType` who can have its sources & sinks appended later
    pub fn new<M, I, II>(id: M, sources: I) -> OrdType
    where
        Match: From<M>,
        Match: From<II>,
        I: IntoIterator<Item = II>,
    {
        let id = Match::from(id);
        OrdType {
            id: id,
            sources: sources.into_iter().map(Match::from).collect(),
            sinks: ExprVec::new(),
        }
    }

    fn source_exists<T>(&self, arg: &T) -> bool
    where
        T: MatchTrait,
    {
        self.sources
            .iter()
            .map(|item| arg.as_ref().eq(item))
            .fold(false, |a, b| a | b)
    }

    fn sink_exists<T>(&self, arg: &T) -> bool
    where
        T: MatchTrait,
    {
        self.sinks
            .iter()
            .map(|item| arg.as_ref().eq(item))
            .fold(false, |a, b| a | b)
    }
}
impl AsRef<Match> for OrdType {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Match {
        &self.id
    }
}
impl MatchTrait for OrdType {}

impl AsRef<OrdType> for OrdType {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a OrdType {
        self
    }
}
impl AsMut<OrdType> for OrdType {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut OrdType {
        self
    }
}

impl PartialEq<TypeData> for &OrdType {
    #[inline(always)]
    fn eq(&self, other: &TypeData) -> bool {
        <OrdType as PartialEq<TypeData>>::eq(*self, other)
    }
}
impl PartialEq<TypeData> for &mut OrdType {
    #[inline(always)]
    fn eq(&self, other: &TypeData) -> bool {
        <OrdType as PartialEq<TypeData>>::eq(*self, other)
    }
}
impl PartialEq<TypeData> for OrdType {
    #[inline(always)]
    fn eq(&self, other: &TypeData) -> bool {
        let item: &Match = self.as_ref();
        item.eq(other)
    }
}
impl OrdTrait for OrdType {}
