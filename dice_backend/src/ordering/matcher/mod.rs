use super::super::parser_output::TypeData;

use super::{OrdTrait, OrdType};

/// Match is our core type used to identify "things to change"
/// it basically gives an expression id `u64` and a `TypeData`
/// which should uniquely identify an expression within the
/// context of its operation.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Match {
    kind: TypeData,
    id: u64,
}

/// MatchTrait is a highlevel trait which permits fetching information about a `Match` item.
pub trait MatchTrait: AsRef<Match> {
    #[inline(always)]
    fn get_kind(&self) -> TypeData {
        self.as_ref().kind
    }

    #[inline(always)]
    fn get_id(&self) -> u64 {
        self.as_ref().id
    }
}

impl AsRef<Match> for Match {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Match {
        self
    }
}

impl MatchTrait for Match {}

/*
 * Various PartialEq of TypeData
 *
 */

impl PartialEq<TypeData> for Match {
    #[inline(always)]
    fn eq(&self, t: &TypeData) -> bool {
        self.kind.eq(t)
    }
}
impl PartialEq<&TypeData> for Match {
    #[inline(always)]
    fn eq(&self, t: &&TypeData) -> bool {
        self.kind.eq(*t)
    }
}
impl PartialEq<TypeData> for &Match {
    #[inline(always)]
    fn eq(&self, t: &TypeData) -> bool {
        self.kind.eq(t)
    }
}

/*
 * Various PartialEq of u64
 *
 */

impl PartialEq<u64> for Match {
    #[inline(always)]
    fn eq(&self, t: &u64) -> bool {
        self.id.eq(t)
    }
}
impl PartialEq<&u64> for Match {
    #[inline(always)]
    fn eq(&self, t: &&u64) -> bool {
        self.id.eq(*t)
    }
}
impl PartialEq<u64> for &Match {
    #[inline(always)]
    fn eq(&self, t: &u64) -> bool {
        self.id.eq(t)
    }
}

impl From<OrdType> for Match {
    #[inline(always)]
    fn from(arg: OrdType) -> Match {
        let id = arg.get_id();
        let kind = arg.get_kind();
        Self { id, kind }
    }
}
/*
impl From<&OrdType> for Match {
    #[inline(always)]
    fn from(arg: &OrdType) -> Match {
        let id = arg.get_id();
        let kind = arg.get_kind();
        Self { id, kind }
    }
}
*/
impl<T: OrdTrait> From<&T> for Match {
    #[inline(always)]
    fn from(arg: &T) -> Match {
        let id = arg.get_id();
        let kind = arg.get_kind();
        Self { id, kind }
    }
}
/*
impl From<Match> for Match {
    #[inline(always)]
    fn from(arg: Match) -> Match {
        arg
    }
}
impl From<&Match> for Match {
    #[inline(always)]
    fn from(arg: &Match) -> Match {
        arg.clone()
    }
}
*/
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
    fn new(id: u64, kind: TypeData) -> Self {
        Self { id, kind }
    }
}
