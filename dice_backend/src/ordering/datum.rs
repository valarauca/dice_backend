use std::hash::{Hash, Hasher};

use super::super::parser_output::TypeData;
use super::super::smallvec::SmallVec;

#[cfg(target_pointer_width = "64")]
pub type IntVec = SmallVec<[i32; 6]>;
#[cfg(target_pointer_width = "32")]
pub type IntVec = SmallVec<[i32; 3]>;
#[cfg(target_pointer_width = "16")]
pub type IntVec = SmallVec<[i32; 1]>;

#[cfg(target_pointer_width = "64")]
pub type BoolVec = SmallVec<[bool; 24]>;
#[cfg(target_pointer_width = "32")]
pub type BoolVec = SmallVec<[bool; 12]>;
#[cfg(target_pointer_width = "16")]
pub type BoolVec = SmallVec<[bool; 6]>;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Datum {
    Bool(bool),
    Int(i32),
    CollectionOfInt(IntVec),
    CollectionOfBool(BoolVec),
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
                state.write_i32(*i);
            }
            &Datum::CollectionOfInt(ref vec) => {
                state.write_u8(3);
                for item in vec.as_slice() {
                    state.write_i32(*item);
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
impl From<i32> for Datum {
    fn from(x: i32) -> Self {
        Self::Int(x)
    }
}
impl From<bool> for Datum {
    fn from(x: bool) -> Self {
        Self::Bool(x)
    }
}
impl From<[i32; 1]> for Datum {
    fn from(x: [i32; 1]) -> Datum {
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
    pub fn get_int(&self) -> i32 {
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

    pub fn sum(&self) -> i32 {
        match self {
            &Datum::CollectionOfInt(ref vec) => vec.as_slice().iter().sum(),
            _ => _unreachable_panic!(),
        }
    }

    pub fn len(&self) -> i32 {
        match self {
            &Datum::CollectionOfInt(ref s) => s.len() as i32,
            &Datum::CollectionOfBool(ref s) => s.len() as i32,
            &Datum::Bool(_) => 1,
            &Datum::Int(_) => 1,
        }
    }

    pub fn extend_from<I: IntoIterator<Item = i32>>(&mut self, arg: I) {
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
    pub fn append_int(&mut self, x: i32) {
        match self {
            &mut Datum::CollectionOfInt(ref mut vec) => {
                vec.push(x);
            }
            _ => _unreachable_panic!(),
        };
    }
}
