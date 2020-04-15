
use super::{DataElement};
use super::super::super::parser_output::TypeData;

/// Tuples are the underlying data type. 
/// Each represents a singular probability outcome. 
#[derive(Clone)]
pub struct Tuple {
    datum: DataElement,
    prob: f64,
}

/*
 * PartialEq/Eq/PartialOrd/Ord
 * only account for the `DataElement`
 * not the ensure structure.
 *
 * This is so identical sets can be merged.
 *
 */
impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        self.datum.eq(&other.datum)
    }
}
impl Eq for Tuple { }
impl PartialOrd for Tuple {
    fn partial_cmp(&self, other: &Tuple) -> Option<::std::cmp::Ordering> {
        self.datum.partial_cmp(&other.datum)
    }
}
impl Ord for Tuple {
    fn cmp(&self, other: &Tuple) -> ::std::cmp::Ordering {
        self.datum.cmp(&other.datum)
    }
}

impl Tuple {

    /// constructor for a constant itn
    pub fn constant_int(x: i32) -> Tuple {
        Tuple {
            datum: DataElement::Int(x),
            prob: 1.0,
        }
    }

    /// constructor for a constant bool
    pub fn constant_bool(x: bool) -> Tuple {
        Tuple {
            datum: DataElement::Bool(x),
            prob: 1.0,
        }
    }


    /*
     * Modifications
     *
     */

    /// sorts the internal representation
    pub fn sort_internal(&mut self) {
        self.datum.sort_internal();
    }

    pub fn get_probability(&self) -> f64 {
        self.prob
    }

    /*
     * Destructor
     *
     */

    pub fn split(self) -> (DataElement,f64) {
        (self.datum,self.prob)
    }

    /*
     * Data Element calls, but re-exposed here
     *
     */
    pub fn get_type(&self) -> TypeData {
        self.datum.get_type()
    }

    pub fn get_bool(&self) -> bool {
        self.datum.get_bool()
    }

    pub fn get_int(&self) -> i32 {
        self.datum.get_int()
    }

    pub fn get_bool_slice<'a>(&'a self) -> &'a [bool] {
        self.datum.get_bool_slice()
    }

    pub fn get_int_slice<'a>(&'a self) -> &'a [i32] {
        self.datum.get_int_slice()
    }
}
