use std::fmt;

use super::typedata::TypeData;

use super::GetType;

/// Literal values fresh from the parser.
///
/// These represent constants like "15" or "true" or "-6".
///
/// There is some fancier types for loading things from the
/// programs environment as well.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal<'a> {
    Number(i64),
    Boolean(bool),
    EnvirBool(&'a str),
    EnvirNumber(&'a str),
}
impl<'a> GetType for Literal<'a> {
    fn requires_namespace(&self) -> bool {
        false
    }

    fn get_type(&self) -> Result<TypeData, String> {
        match self {
            Literal::Number(_) | Literal::EnvirNumber(_) => Ok(TypeData::Int),
            Literal::Boolean(_) | Literal::EnvirBool(_) => Ok(TypeData::Bool),
        }
    }
}
impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(ref num) => write!(f, "{}", *num),
            Literal::Boolean(ref var) => {
                if *var {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Literal::EnvirBool(ref name) => write!(f, "%b{{{{{}}}}}", name),
            Literal::EnvirNumber(ref name) => write!(f, "%d{{{{{}}}}}", name),
        }
    }
}

#[test]
fn test_literal_parsing() {
    use super::super::value::LitParser;

    let parser = LitParser::new();
    assert!(parser.parse("false").unwrap() == Literal::Boolean(false));
    assert!(parser.parse("true").unwrap() == Literal::Boolean(true));
    assert!(parser.parse("15").unwrap() == Literal::Number(15i64));
    assert!(parser.parse("-30").unwrap() == Literal::Number(-30i64));
    assert!(parser.parse("%d{{ENV_VAR}}").unwrap() == Literal::EnvirNumber("ENV_VAR"));
    assert!(parser.parse("%b{{ENV_VAR}}").unwrap() == Literal::EnvirBool("ENV_VAR"));
}
