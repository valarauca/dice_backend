use super::super::parser_output::TypeData;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Datum {
    Bool(bool),
    Int(i32),
    CollectionOfInt(Vec<i32>),
    CollectionOfBool(Vec<bool>),
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
impl From<Vec<i32>> for Datum {
    fn from(x: Vec<i32>) -> Self {
        Self::CollectionOfInt(x)
    }
}
impl From<Vec<bool>> for Datum {
    fn from(x: Vec<bool>) -> Self {
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

    pub fn get_bool_vec<'a>(&'a self) -> &'a [bool] {
        match self {
            &Datum::CollectionOfBool(ref vec) => vec.as_slice(),
            _ => _unreachable_panic!(),
        }
    }

    pub fn get_int_vec<'a>(&'a self) -> &'a [i32] {
        match self {
            &Datum::CollectionOfInt(ref vec) => vec.as_slice(),
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
            &mut Datum::CollectionOfInt(ref mut vec) => vec.sort_unstable(),
            &mut Datum::CollectionOfBool(ref mut vec) => vec.sort_unstable(),
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
