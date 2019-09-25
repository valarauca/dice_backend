use std::hash::{Hash, Hasher};

use super::super::seahash::SeaHasher;

/// Identifier is used to identify a value.
/// That value may-or-may-not be within a
/// namespace (function body) therefore there
/// are two variants of identification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Identifier {
    Global(u64),
    Scoped(u64, u64),
}
impl Identifier {
    pub fn new(namespace: Option<&str>, name: &str) -> Identifier {
        match namespace {
            Option::None => Identifier::new_global(name),
            Option::Some(value) => Identifier::new_scoped(value, name),
        }
    }

    fn new_scoped(namespace: &str, name: &str) -> Identifier {
        let namespace: u64 = {
            let mut seahasher = SeaHasher::default();
            namespace.hash(&mut seahasher);
            seahasher.finish()
        };
        let name: u64 = {
            let mut seahasher = SeaHasher::default();
            name.hash(&mut seahasher);
            seahasher.finish()
        };
        Identifier::Scoped(namespace, name)
    }

    fn new_global(name: &str) -> Identifier {
        let name: u64 = {
            let mut seahasher = SeaHasher::default();
            name.hash(&mut seahasher);
            seahasher.finish()
        };
        Identifier::Global(name)
    }

    pub fn is_scoped(&self) -> bool {
        match self {
            &Identifier::Scoped(_, _) => true,
            _ => false,
        }
    }

    pub fn is_global(&self) -> bool {
        !self.is_scoped()
    }
}
