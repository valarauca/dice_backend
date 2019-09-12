
use super::super::parser_output::{Literal,TypeData,Operation};

use super::identifier::Identifier;

/// LinkageExpression is the results of identifiers being removed.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum LinkageExpression<'a> {
    Constant(Literal<'a>,TypeData),
    Operation(Identifier,Operation,Identifier,TypeData),
    Function(u64,Box<[Identifier]>,TypeData),
}
