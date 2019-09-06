use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::replace;

use super::super::parser_output::{Expression, GetType, Literal, Operation, TypeData};
use super::super::seahash::SeaHasher;

/// BlockExpression is the result of expresion lowering.
/// when preformed it. Block Expressions, unlike normal
/// expressions are not a recrusive data type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockExpression<'a> {
    FunctionArg(&'a str, TypeData),
    ConstantValue(Literal<'a>, TypeData),
    ExternalConstant(&'a str, TypeData),
    Func(&'a str, Box<[BlockExpression<'a>]>, TypeData),
    Var(&'a str, TypeData),
    Op(
        Box<BlockExpression<'a>>,
        Operation,
        Box<BlockExpression<'a>>,
    ),
}
impl<'a> fmt::Display for BlockExpression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockExpression::ConstantValue(ref lit, _) => write!(f, "{}", lit),
            BlockExpression::ExternalConstant(ref name, _) => write!(f, "{}", name),
            BlockExpression::FunctionArg(ref name, _) => write!(f, "{}", name),
            BlockExpression::Var(ref name, _) => write!(f, "{}", name),
            BlockExpression::Op(ref left, ref op, ref right) => {
                write!(f, "( {} {} {} )", left, op, right)
            }
            BlockExpression::Func(ref name, ref args, _) => {
                write!(f, "{}(", name)?;
                let last_arg = args.len() - 1;
                for (index, arg) in args.iter().enumerate() {
                    if index == last_arg {
                        write!(f, "{}", arg)?;
                    } else {
                        write!(f, "{},", arg)?;
                    }
                }
                write!(f, ")")
            }
        }
    }
}
impl<'a> GetType for BlockExpression<'a> {
    fn requires_namespace(&self) -> bool {
        false
    }

    /// resolving the typing data for the block expression
    fn get_type(&self) -> Result<TypeData, String> {
        match self {
            BlockExpression::ConstantValue(_, kind) => Ok(kind.clone()),
            BlockExpression::ExternalConstant(_, kind) => Ok(kind.clone()),
            BlockExpression::FunctionArg(_, kind) => Ok(kind.clone()),
            BlockExpression::Func(_, _, kind) => Ok(kind.clone()),
            BlockExpression::Var(_, kind) => Ok(kind.clone()),
            BlockExpression::Op(ref left, op, ref right) => match op {
                Operation::Sub | Operation::Mul | Operation::Div | Operation::Add => {
                    match (left.get_type()?, right.get_type()?) {
                        (TypeData::Int, TypeData::Int) => Ok(TypeData::Int),
                        (TypeData::Int, TypeData::CollectionOfInt) => Ok(TypeData::CollectionOfInt),
                        (TypeData::CollectionOfInt, TypeData::Int) => Ok(TypeData::Int),
                        (TypeData::CollectionOfInt, TypeData::CollectionOfInt) => {
                            Ok(TypeData::CollectionOfInt)
                        }
                        // TODO this sucks
                        (left_type, right_type) => Err(format!(
                            "Type Error. Expression: ({} {} {}) is illegal. {} cannot {} with {}",
                            left, op, right, left_type, op, right_type
                        )),
                    }
                }
                Operation::Equal
                | Operation::GreaterThan
                | Operation::LessThan
                | Operation::GreaterThanEqual
                | Operation::LessThanEqual => match (left.get_type()?, right.get_type()?) {
                    (TypeData::Int, TypeData::Int) => Ok(TypeData::Bool),
                    (TypeData::Int, TypeData::CollectionOfInt) => Ok(TypeData::CollectionOfBool),
                    (TypeData::CollectionOfInt, TypeData::Int) => Ok(TypeData::CollectionOfBool),
                    (TypeData::CollectionOfInt, TypeData::CollectionOfInt) => {
                        Ok(TypeData::CollectionOfBool)
                    }
                    // TODO this sucks
                    (left, right) => Err(format!(
                        "type error. Expression on ({} {} {}) is illegal",
                        left, op, right
                    )),
                },
                Operation::Or | Operation::And => match (left.get_type()?, right.get_type()?) {
                    (TypeData::Bool, TypeData::Bool) => Ok(TypeData::Bool),
                    (TypeData::Bool, TypeData::CollectionOfBool) => Ok(TypeData::CollectionOfBool),
                    (TypeData::CollectionOfBool, TypeData::Bool) => Ok(TypeData::CollectionOfBool),
                    (TypeData::CollectionOfBool, TypeData::CollectionOfBool) => {
                        Ok(TypeData::CollectionOfBool)
                    }
                    // TODO this sucks
                    (left, right) => Err(format!(
                        "type error. Expression on ({} {} {}) is illegal",
                        left, op, right
                    )),
                },
            },
        }
    }
}
