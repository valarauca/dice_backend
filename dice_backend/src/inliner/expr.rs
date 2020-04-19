use std::hash::{Hash, Hasher};
use std::str::FromStr;

use super::super::cfgbuilder::{CallStack, ExpressionCollection, HashedExpression, Identifier};
use super::super::parser_output::{Literal, Operation, TypeData};
use super::super::seahash::SeaHasher;

use super::coll::InlinedCollection;

/// Inlined Expression contains the very base values
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InlinedExpression {
    D6(u64),
    D3(u64),
    Filter(u64, u64),
    Count(u64),
    Len(u64),
    Join(u64, u64),
    Sum(u64),
    Operation(u64, Operation, u64, TypeData),
    ConstantInt(i32),
    ConstantBool(bool),
}
impl<'a> InlinedExpression {
    /// returns the hash of the expression
    pub fn get_hash(&self) -> u64 {
        let mut hasher = SeaHasher::default();
        self.hash(&mut hasher);
        hasher.finish()
    }
    /// Build an inlined expression from context
    pub fn new<'b>(
        expr: &'b HashedExpression<'a>,
        stack: &mut CallStack<'a, 'b>,
        coll: &mut InlinedCollection,
    ) -> InlinedExpression {
        let hash = expr.get_hash();
        let output = match expr {
            &HashedExpression::ConstantValue(Literal::EnvirBool(ref envir_name), _) => {
                let b = ::std::env::vars()
                    .filter(|(name, _)| envir_name == name)
                    .flat_map(|(_, var)| bool::from_str(&var).ok())
                    .next()
                    .expect(&format!(
                        "could not fine value {} in environment",
                        envir_name
                    ));
                InlinedExpression::ConstantBool(b)
            }
            &HashedExpression::ConstantValue(Literal::EnvirNumber(ref envir_name), _) => {
                let i = ::std::env::vars()
                    .filter(|(name, _)| envir_name == name)
                    .flat_map(|(_, var)| i32::from_str(&var).ok())
                    .next()
                    .expect(&format!(
                        "could not fine value {} in environment",
                        envir_name
                    ));
                InlinedExpression::ConstantInt(i)
            }
            &HashedExpression::ConstantValue(Literal::Number(i), _) => {
                InlinedExpression::ConstantInt(i as i32)
            }
            &HashedExpression::ConstantValue(Literal::Boolean(b), _) => {
                InlinedExpression::ConstantBool(b)
            }
            &HashedExpression::ExternalConstant(ref id, _) | &HashedExpression::Var(ref id, _) => {
                // resolve the expression that defines the variable
                // convert that recursively
                InlinedExpression::variable(id, stack, coll)
            }
            &HashedExpression::FunctionArg(_, ref index, _) => {
                // compose a function argument into
                InlinedExpression::func_arg(index, stack, coll)
            }
            &HashedExpression::Func(ref id, ref args, ref kind) => {
                InlinedExpression::func(id, args.as_ref(), &hash, stack, coll, kind)
            }
            &HashedExpression::Op(ref left, op, ref right, out) => {
                // convert arguments into new format
                let left = InlinedExpression::new(stack.get_expr(left).unwrap(), stack, coll);
                let right = InlinedExpression::new(stack.get_expr(right).unwrap(), stack, coll);
                match (left, right) {
                    (InlinedExpression::ConstantBool(l), InlinedExpression::ConstantBool(r)) => {
                        match (out, op) {
                            (TypeData::Bool, Operation::And) => {
                                InlinedExpression::ConstantBool(l & r)
                            }
                            (TypeData::Bool, Operation::Or) => {
                                InlinedExpression::ConstantBool(l | r)
                            }
                            _ => panic!("other boolean expressions are not possible"),
                        }
                    }
                    (
                        InlinedExpression::ConstantInt(left),
                        InlinedExpression::ConstantInt(right),
                    ) => match (out, op) {
                        (TypeData::Int, Operation::Add) => {
                            InlinedExpression::ConstantInt(left + right)
                        }
                        (TypeData::Int, Operation::Sub) => {
                            InlinedExpression::ConstantInt(left - right)
                        }
                        (TypeData::Int, Operation::Mul) => {
                            InlinedExpression::ConstantInt(left * right)
                        }
                        (TypeData::Int, Operation::Div) => {
                            InlinedExpression::ConstantInt(left / right)
                        }
                        (TypeData::Int, Operation::Or) => {
                            InlinedExpression::ConstantInt(left | right)
                        }
                        (TypeData::Int, Operation::And) => {
                            InlinedExpression::ConstantInt(left & right)
                        }
                        (TypeData::Bool, Operation::Equal) => {
                            InlinedExpression::ConstantBool(left == right)
                        }
                        (TypeData::Bool, Operation::GreaterThan) => {
                            InlinedExpression::ConstantBool(left > right)
                        }
                        (TypeData::Bool, Operation::LessThan) => {
                            InlinedExpression::ConstantBool(left < right)
                        }
                        (TypeData::Bool, Operation::GreaterThanEqual) => {
                            InlinedExpression::ConstantBool(left >= right)
                        }
                        (TypeData::Bool, Operation::LessThanEqual) => {
                            InlinedExpression::ConstantBool(left <= right)
                        }
                        _ => panic!("illegal interger operation"),
                    },
                    (left, right) => {
                        InlinedExpression::Operation(left.get_hash(), op, right.get_hash(), out)
                    }
                    anything_else => _unreachable_panic!(
                        "illegal operation. Should be caught by type checker. {:?}",
                        anything_else
                    ),
                }
            }
        };
        coll.insert_hash(&output);
        output
    }

    /*
     * Private implemenation details to avoid the match statement from getting extremely bloated
     *
     */

    #[inline(always)]
    fn variable<'b>(
        id: &Identifier,
        stack: &mut CallStack<'a, 'b>,
        coll: &mut InlinedCollection,
    ) -> InlinedExpression {
        stack
            .get_var(id)
            .into_iter()
            .map(|expr| InlinedExpression::new(expr, stack, coll))
            .next()
            .unwrap()
    }

    #[inline(always)]
    fn func_arg<'b>(
        arg_index: &usize,
        stack: &mut CallStack<'a, 'b>,
        coll: &mut InlinedCollection,
    ) -> InlinedExpression {
        let context = stack.get_context().unwrap();
        let func_expr = stack.get_ctx_expr().unwrap();
        let arg_expr = stack.get_arg_index(*arg_index).unwrap();
        stack.pop();
        let out = InlinedExpression::new(stack.get_expr(&arg_expr).unwrap(), stack, coll);
        stack.push(&context, &func_expr);
        out
    }

    #[inline(always)]
    fn func<'b>(
        id: &Identifier,
        args: &[u64],
        hash: &u64,
        stack: &mut CallStack<'a, 'b>,
        coll: &mut InlinedCollection,
        kind: &TypeData,
    ) -> InlinedExpression {
        if stack.is_stdlib(id) {
            match stack.get_function_name(id).unwrap() {
                "roll_d6" => {
                    debug_assert_eq!(*kind, TypeData::CollectionOfInt);
                    debug_assert_eq!(args.len(), 1);
                    let expr = stack.get_expr(&args[0]).unwrap();
                    debug_assert_eq!(expr.get_type(), TypeData::Int);
                    let arg = InlinedExpression::new(expr, stack, coll);
                    InlinedExpression::D6(arg.get_hash())
                }
                "roll_d3" => {
                    debug_assert_eq!(*kind, TypeData::CollectionOfInt);
                    debug_assert_eq!(args.len(), 1);
                    let expr = stack.get_expr(&args[0]).unwrap();
                    debug_assert_eq!(expr.get_type(), TypeData::Int);
                    let arg = InlinedExpression::new(expr, stack, coll);
                    InlinedExpression::D3(arg.get_hash())
                }
                "filter" => {
                    debug_assert_eq!(*kind, TypeData::CollectionOfInt);
                    debug_assert_eq!(args.len(), 2);

                    // filter arg 1
                    let expr1 = stack.get_expr(&args[0]).unwrap();
                    debug_assert_eq!(expr1.get_type(), TypeData::CollectionOfBool);
                    let arg1 = InlinedExpression::new(expr1, stack, coll);

                    // filter arg 2
                    let expr2 = stack.get_expr(&args[1]).unwrap();
                    debug_assert_eq!(expr2.get_type(), TypeData::CollectionOfInt);
                    let arg2 = InlinedExpression::new(expr2, stack, coll);

                    InlinedExpression::Filter(arg1.get_hash(), arg2.get_hash())
                }
                "count" => {
                    debug_assert_eq!(*kind, TypeData::Int);
                    debug_assert_eq!(args.len(), 1);
                    let expr = stack.get_expr(&args[0]).unwrap();
                    debug_assert_eq!(expr.get_type(), TypeData::CollectionOfBool);
                    let arg = InlinedExpression::new(expr, stack, coll);
                    InlinedExpression::Count(arg.get_hash())
                }
                "len" => {
                    debug_assert_eq!(*kind, TypeData::Int);
                    debug_assert_eq!(args.len(), 1);
                    let expr = stack.get_expr(&args[0]).unwrap();
                    debug_assert_eq!(expr.get_type(), TypeData::CollectionOfInt);
                    let arg = InlinedExpression::new(expr, stack, coll);
                    InlinedExpression::Len(arg.get_hash())
                }
                "join" => {
                    debug_assert_eq!(*kind, TypeData::CollectionOfInt);
                    debug_assert_eq!(args.len(), 2);

                    // join arg 1
                    let expr1 = stack.get_expr(&args[0]).unwrap();
                    debug_assert_eq!(expr1.get_type(), TypeData::CollectionOfInt);
                    let arg1 = InlinedExpression::new(expr1, stack, coll);

                    // join arg 2
                    let expr2 = stack.get_expr(&args[1]).unwrap();
                    debug_assert_eq!(expr2.get_type(), TypeData::CollectionOfInt);
                    let arg2 = InlinedExpression::new(expr2, stack, coll);

                    InlinedExpression::Join(arg1.get_hash(), arg2.get_hash())
                }
                "sum" => {
                    debug_assert_eq!(*kind, TypeData::Int);
                    debug_assert_eq!(args.len(), 1);
                    let expr = stack.get_expr(&args[0]).unwrap();
                    debug_assert_eq!(expr.get_type(), TypeData::CollectionOfInt);
                    let arg = InlinedExpression::new(expr, stack, coll);
                    InlinedExpression::Sum(arg.get_hash())
                }
                _ => panic!("item is not a part of the standard library"),
            }
        } else {
            stack.push(id, &hash);
            let output = stack
                .get_return()
                .into_iter()
                .map(|expr| InlinedExpression::new(expr, stack, coll))
                .next()
                .unwrap();
            stack.pop();
            output
        }
    }
}
