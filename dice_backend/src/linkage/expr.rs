
use super::super::parser_output::{Literal,TypeData,Operation};
use super::super::cfgbuilder::{HashedExpression,ExpressionCollection,Identifier};

/// LinkageExpression is the results of identifiers being removed.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum LinkageExpression<'a> {
    Constant(Literal<'a>,TypeData),
    Operation(Identifier,Operation,Identifier,TypeData),
    Function(u64,Box<[Identifier]>,TypeData),
}
impl LinkageExpression<'a> {

    pub fn expr(expression: &HashedExpression<'a>, n: &ExpressionCollection<'a>) -> LinkageExpression<'a> {
        match expression {
            &HashedExpression::ConstantValue(ref lit, ref kind) => LinkageExpression::Constant(lit.clone(), kind.clone()),
            &HashedExpression::ExternalConstant(ref id, ref kind) => {
            }
        }
    }
}
