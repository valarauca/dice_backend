use super::super::super::runner::InlinedExpression;

/// Hashing store information about hashing
///
/// NOTE:
///
/// This hash value is actually derived from `InlinedExpression`
/// and is as of this pass actually just an opaque value associated
/// with the underlying term.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hash {
    hash_value: u64,
}
impl<'a, 'b> From<&'b InlinedExpression<'a>> for Hash {
    fn from(arg: &'b InlinedExpression<'a>) -> Hash {
        Hash {
            hash_value: arg.get_hash(),
        }
    }
}
impl AsRef<Hash> for Hash {
    fn as_ref<'a>(&'a self) -> &'a Hash {
        self
    }
}
impl HashOp for Hash {}

/// HashOp is a trait which implements most of the interesting
/// effects of the hash comparisons.
pub trait HashOp: AsRef<Hash> {
    fn get_hash(&self) -> u64 {
        self.as_ref().hash_value.clone()
    }

    fn equal_hash<T: HashOp>(&self, other: &T) -> bool {
        self.get_hash() == other.get_hash()
    }
}
