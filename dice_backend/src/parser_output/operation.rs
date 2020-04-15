use std::fmt;

/// Operations are things we do to numbers
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Or,
    And,
    Equal,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    NotEqual,
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Sub => write!(f, "-"),
            Operation::Mul => write!(f, "*"),
            Operation::Div => write!(f, "/"),
            Operation::Or => write!(f, "|"),
            Operation::And => write!(f, "&"),
            Operation::Equal => write!(f, "=="),
            Operation::GreaterThan => write!(f, ">"),
            Operation::LessThan => write!(f, "<"),
            Operation::GreaterThanEqual => write!(f, ">="),
            Operation::LessThanEqual => write!(f, "<="),
            Operation::NotEqual => write!(f,"!="),
        }
    }
}

#[test]
fn test_operation_parsing() {
    use super::super::value::OpParser;

    let parser = OpParser::new();
    assert!(parser.parse("+").unwrap() == Operation::Add);
    assert!(parser.parse("-").unwrap() == Operation::Sub);
    assert!(parser.parse("*").unwrap() == Operation::Mul);
    assert!(parser.parse("/").unwrap() == Operation::Div);
}
