use std::fmt;
use std::hash::{Hash, Hasher};

use super::super::parser_output::TypeData;
use super::super::smallvec::SmallVec;

pub type IntVec = SmallVec<[i8; 24]>;

pub type BoolVec = SmallVec<[bool; 24]>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Datum {
    Bool(bool),
    Int(i8),
    CollectionOfInt(IntVec),
    CollectionOfBool(BoolVec),
}
#[test]
fn assert_datum_size() {
    assert_eq!(::std::mem::size_of::<Datum>(), 48);
}
impl fmt::Debug for Datum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Datum::Bool(ref b) => write!(f, "{}", *b),
            &Datum::Int(ref i) => write!(f, "{}", *i),
            &Datum::CollectionOfInt(ref i) => {
                f.debug_list().entries(i.clone().into_iter()).finish()
            }
            &Datum::CollectionOfBool(ref b) => {
                f.debug_list().entries(b.clone().into_iter()).finish()
            }
        }
    }
}
impl fmt::Display for Datum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Datum as fmt::Debug>::fmt(self, f)
    }
}
impl Hash for Datum {
    // implement Hash myself because I saw a spooky collision yesterday
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            &Datum::Bool(ref b) => {
                state.write_u8(1);
                state.write_u8(*b as u8);
            }
            &Datum::Int(ref i) => {
                state.write_u8(2);
                state.write_i8(*i);
            }
            &Datum::CollectionOfInt(ref vec) => {
                state.write_u8(3);
                for item in vec.as_slice() {
                    state.write_i8(*item);
                }
            }
            &Datum::CollectionOfBool(ref vec) => {
                state.write_u8(4);
                for item in vec.as_slice() {
                    state.write_u8(*item as u8);
                }
            }
        }
    }
}
impl From<i8> for Datum {
    fn from(x: i8) -> Self {
        Self::Int(x)
    }
}
impl From<bool> for Datum {
    fn from(x: bool) -> Self {
        Self::Bool(x)
    }
}
impl From<[i8; 1]> for Datum {
    fn from(x: [i8; 1]) -> Datum {
        let mut smol_vec = IntVec::new();
        smol_vec.extend_from_slice(&x);
        Self::CollectionOfInt(smol_vec)
    }
}
impl From<IntVec> for Datum {
    fn from(x: IntVec) -> Datum {
        Self::CollectionOfInt(x)
    }
}
impl From<[bool; 1]> for Datum {
    fn from(x: [bool; 1]) -> Datum {
        let mut smol_vec = BoolVec::new();
        smol_vec.extend_from_slice(&x);
        Self::CollectionOfBool(smol_vec)
    }
}
impl From<BoolVec> for Datum {
    fn from(x: BoolVec) -> Self {
        Self::CollectionOfBool(x)
    }
}
impl Datum {
    pub fn get_type(&self) -> TypeData {
        match self {
            &Datum::CollectionOfInt(_) => TypeData::CollectionOfInt,
            &Datum::CollectionOfBool(_) => TypeData::CollectionOfBool,
            &Datum::Bool(_) => TypeData::Bool,
            &Datum::Int(_) => TypeData::Int,
        }
    }

    /// return datum as an int
    pub fn get_int(&self) -> i8 {
        match self {
            &Datum::Int(ref i) => i.clone(),
            _ => _unreachable_panic!(),
        }
    }

    pub fn get_bool(&self) -> bool {
        match self {
            &Datum::Bool(ref b) => b.clone(),
            _ => _unreachable_panic!(),
        }
    }

    pub fn get_bool_vec(self) -> BoolVec {
        match self {
            Datum::CollectionOfBool(vec) => vec,
            _ => _unreachable_panic!(),
        }
    }

    pub fn get_int_vec(self) -> IntVec {
        match self {
            Datum::CollectionOfInt(vec) => vec,
            _ => _unreachable_panic!(),
        }
    }

    pub fn sum(&self) -> i8 {
        match self {
            &Datum::CollectionOfInt(ref vec) => vec.as_slice().iter().sum(),
            _ => _unreachable_panic!(),
        }
    }

    pub fn len(&self) -> i8 {
        match self {
            &Datum::CollectionOfInt(ref s) => s.len() as i8,
            &Datum::CollectionOfBool(ref s) => s.len() as i8,
            &Datum::Bool(_) => 1,
            &Datum::Int(_) => 1,
        }
    }

    pub fn extend_from<I: IntoIterator<Item = i8>>(&mut self, arg: I) {
        match self {
            &mut Datum::CollectionOfInt(ref mut vec) => vec.extend(arg),
            _ => _unreachable_panic!(),
        };
    }

    pub fn sort(&mut self) {
        match self {
            &mut Datum::CollectionOfInt(ref mut vec) => vec.as_mut_slice().sort_unstable(),
            &mut Datum::CollectionOfBool(ref mut vec) => vec.as_mut_slice().sort_unstable(),
            _ => {}
        }
    }

    /// appends an int
    pub fn append_int(&mut self, x: i8) {
        match self {
            &mut Datum::CollectionOfInt(ref mut vec) => {
                vec.push(x);
            }
            _ => _unreachable_panic!(),
        };
    }
}
