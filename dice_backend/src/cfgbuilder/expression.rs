
use std::hash::{Hasher,Hash};

use super::super::parser_output::{
    Expression, FunctionInvocation, GetType, Literal, Operation, TypeData,
};
use super::super::seahash::SeaHasher;

use super::identifier::{Identifier};

/// HashedExpressions are built ontop of BlockExpressions. 
/// They're goal is to start linking together the overal
/// program's structure. 
///
/// Since `BlockExpression` & `BasicBlock` have handled
/// all of our type errors & namespace errors we can
/// convert our expressions into namespace specific
/// hashes, and shove everything into local tables.
///
/// Since names are immutable, and defined only once hashing
/// is all we need.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum HashedExpression<'a> {
    FunctionArg(Identifier, TypeData),
    ConstantValue(Literal<'a>,TypeData),
    ExternalConstant(Identifier, TypeData),
    Var(Identifier, TypeData),
    Func(Identifier, Box<[u64]>, TypeData),
    Op(
        u64,
        Operation,
        u64,
        TypeData,
    ),
}
impl<'a> HashedExpression<'a> {

    pub fn get_type(&self) -> TypeData {
        match self {
            HashedExpression::FunctionArg(_, ref op) => op.clone(),
            HashedExpression::ConstantValue(_, ref op) => op.clone(),
            HashedExpression::ExternalConstant(_, ref op) => op.clone(),
            HashedExpression::Var(_, ref op) => op.clone(),
            HashedExpression::Func(_, _, ref op) => op.clone(),
            HashedExpression::Op(_,_,_,ref op) => op.clone()
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = SeaHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
