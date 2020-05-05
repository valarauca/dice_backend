use super::super::super::ordering::OrdTrait;
use super::super::super::parser_output::TypeData;

/// Match is our core type used to identify "things to change"
/// it basically gives an expression id `u64` and a `TypeData`
/// which should uniquely identify an expression within the
/// context of its operation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Match {
    kind: Option<TypeData>,
    id: Option<u64>,
}
impl Match {
    pub fn get_kind(&self) -> Option<TypeData> {
        self.kind.clone()
    }
    pub fn get_id(&self) -> Option<u64> {
        self.id.clone()
    }

    pub fn none() -> Match {
        Match {
            id: None,
            kind: None,
        }
    }
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
        let (id, kind) = (Some(id), Some(kind));
        Self { id, kind }
    }
}
impl From<&(u64, TypeData)> for Match {
    #[inline(always)]
    fn from(arg: &(u64, TypeData)) -> Self {
        let (id, kind) = *arg;
        let (id, kind) = (Some(id), Some(kind));
        Self { id, kind }
    }
}
impl From<(u64, TypeData)> for Match {
    #[inline(always)]
    fn from(arg: (u64, TypeData)) -> Self {
        let (id, kind) = arg;
        let (id, kind) = (Some(id), Some(kind));
        Self { id, kind }
    }
}
impl Match {
    #[inline(always)]
    pub fn new(id: u64, kind: TypeData) -> Self {
        let (id, kind) = (Some(id), Some(kind));
        Self { id, kind }
    }
}
