
use super::super::smallvec::{SmallVec};

use super::super::parser_output::{TypeData};

/// A collection of expressions that -this- expression requires
/// as arguments, or the expressions which require -this- as
/// an argument.
pub type ExprVec = SmallVec<[(u64,TypeData);2]>;

/// OrdTrait contains information about the ordering of an expression
pub trait OrdTrait: AsRef<OrdType> + AsMut<OrdType> {

    /// returns the expression id of -this- expression.
    fn own_id(&self) -> u64 {
        self.as_ref().self_id.clone()
    }

    /// returns the type of -this- expression.
    fn get_own_type(&self) -> TypeData {
        self.as_ref().self_type.clone()
    }

    /// returns what this expression must read
    fn get_sources<'a>(&'a self) -> &'a [(u64,TypeData)] {
        self.as_ref().sources.as_slice()
    }

    /// are there any sources?
    fn has_sources(&self) -> bool {
        self.get_sources().len() > 1
    }

    /// returns where this expression results;
    fn get_sinks<'a>(&'a self) -> &'a [(u64,TypeData)] {
        self.as_ref().sinks.as_slice()
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

    fn add_sink(&mut self, id: u64, kind: TypeData) {
        // this must be true
        debug_assert_eq!(self.get_own_type(), kind);

        if ! self.as_ref().sink_exists(id, kind) {
            self.as_mut().sinks.push( (id,kind) );
        }
    }
}

/// OrdType encodes information about ordering.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct OrdType {
    self_id: u64,
    self_type: TypeData,
    sources: ExprVec,
    sinks: ExprVec,
}
impl OrdType {

    /// build a new `OrdType` who can have its sources & sinks appended later
    pub fn new(
        self_id: u64,
        self_type: TypeData,
        sources: ExprVec,
    ) -> OrdType {
        OrdType {
            self_id: self_id,
            self_type: self_type,
            sources: sources.into_iter().collect(),
            sinks: ExprVec::new(),
        }
    }

    fn source_exists(&self, id: u64, kind: TypeData) -> bool {
        self.sources.iter().map(|(expr_id, expr_kind)| *expr_id == id && *expr_kind == kind).fold(false,|a,b| a | b)
    }

    fn sink_exists(&self, id: u64, kind: TypeData) -> bool {
        self.sinks.iter().map(|(expr_id, expr_kind)| *expr_id == id && *expr_kind == kind).fold(false,|a,b| a | b)
    }
}
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
impl OrdTrait for OrdType { }

