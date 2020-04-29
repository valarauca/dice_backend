use super::super::super::ordering::OrdTrait;
use super::super::super::parser_output::TypeData;

/// Match is our core type used to identify "things to change"
/// it basically gives an expression id `u64` and a `TypeData`
/// which should uniquely identify an expression within the
/// context of its operation.
#[repr(packed)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Match {
    pub kind: TypeData,
    pub id: u64,
}
impl<T: OrdTrait> From<&T> for Match {
    #[inline(always)]
    fn from(arg: &T) -> Self {
        Self::from(arg.get_matcher_tuple())
    }
}
impl From<&&(u64, TypeData)> for Match {
    #[inline(always)]
    fn from(arg: &&(u64, TypeData)) -> Self {
        let (id, kind) = **arg;
        Self { id, kind }
    }
}
impl From<&(u64, TypeData)> for Match {
    #[inline(always)]
    fn from(arg: &(u64, TypeData)) -> Self {
        let (id, kind) = *arg;
        Self { id, kind }
    }
}
impl From<(u64, TypeData)> for Match {
    #[inline(always)]
    fn from(arg: (u64, TypeData)) -> Self {
        let (id, kind) = arg;
        Self { id, kind }
    }
}
impl Match {
    #[inline(always)]
    pub fn new(id: u64, kind: TypeData) -> Self {
        Self { id, kind }
    }
}
