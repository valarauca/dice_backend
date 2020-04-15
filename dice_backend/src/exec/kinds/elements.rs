
use super::super::super::parser_output::TypeData;

/// DataElement is "the value" contained within a
/// a tuple.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum DataElement {
    Bool(bool),
    Int(i32),
    CollofInt(Vec<i32>),
    CollofBool(Vec<bool>),
}
impl DataElement {

    /// sorts internal data type.
    ///
    /// Does nothing for the `null`, `bool`, and `int` variants.
    /// performs unstable sort on the `CollOfWhatever` variants.
    pub fn sort_internal(&mut self) {
        match self {
            &mut DataElement::Bool(_) | &mut DataElement::Int(_) => { },
            &mut DataElement::CollofInt(ref mut vec) => {
                vec.sort_unstable();
            },
            &mut DataElement::CollofBool(ref mut vec) => {
                vec.sort_unstable();
            },
        }
    }

    /// push an int to the internal collection, fails if internal is not an int
    pub fn push_int(&mut self, x: i32) {
        match self {
            &mut DataElement::CollofInt(ref mut vec) => {
                vec.push(x);
            },
            z => panic!("expected collection of int, found {:?}", z)
        }
    }

    pub fn get_int_slice<'a>(&'a self) -> &'a [i32] {
        match self {
            &DataElement::CollofInt(ref vec) => vec.as_slice(),
            z => panic!("expected collection of int, found {:?}", z)
        }
    }

    /// push booleans into a collection
    pub fn push_bool(&mut self, b: bool) {
        match self {
            &mut DataElement::CollofBool(ref mut vec) => {
                vec.push(b);
            },
            z => panic!("expected collection of bool, found {:?}", z)
        }
    }

    pub fn get_bool_slice<'a>(&'a self) -> &'a [bool] {
        match self {
            &DataElement::CollofBool(ref vec) => vec.as_slice(),
            z => panic!("expected collection of bool, found {:?}", z)
        }
    }

    /// returns the boolean value, panics if not a boolean
    pub fn get_bool(&self) -> bool {
        match self {
            &DataElement::Bool(b) => b,
            x => panic!("type error. Expected a boolean, found {:?}", x)
        }
    }

    /// returns an interger value, panics if not an int.
    pub fn get_int(&self) -> i32 {
        match self {
            &DataElement::Int(x) => x,
            z => panic!("type error. Expected an int, found {:?}", z),
        }
    }

    /// returns the type of the element
    pub fn get_type(&self) -> TypeData {
        match self {
            &DataElement::Bool(_) => TypeData::Bool,
            &DataElement::Int(_) => TypeData::Int,
            &DataElement::CollofInt(_) => TypeData::CollectionOfInt,
            &DataElement::CollofBool(_) => TypeData::CollectionOfBool,
        }
    }
}
