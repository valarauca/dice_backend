use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeData {
    Bool,
    Int,
    CollectionOfBool,
    CollectionOfInt,
}
impl fmt::Display for TypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeData::Bool => write!(f, "bool"),
            TypeData::Int => write!(f, "int"),
            TypeData::CollectionOfBool => write!(f, "vec<bool>"),
            TypeData::CollectionOfInt => write!(f, "vec<int>"),
        }
    }
}

#[test]
fn test_type_data_parsing() {
    use super::super::value::KindParser;

    let parser = KindParser::new();
    assert!(parser.parse("bool").unwrap() == TypeData::Bool);
    assert!(parser.parse("int").unwrap() == TypeData::Int);
    assert!(parser.parse("vec<bool>").unwrap() == TypeData::CollectionOfBool);
    assert!(parser.parse("vec<int>").unwrap() == TypeData::CollectionOfInt);
}

