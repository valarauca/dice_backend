use std::hash::{Hash, Hasher};
use std::str::FromStr;

use super::super::cfgbuilder::{CallStack, ExpressionCollection, HashedExpression, Identifier};
use super::super::parser_output::{Literal, Operation, TypeData};
use super::super::seahash::SeaHasher;

use super::coll::InlinedCollection;

/// Inlined Expression contains the very base values
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InlinedExpression<'a> {
    StdLibFunc(&'a str, Box<[u64]>),
    Operation(u64, Operation, u64, TypeData),
    ConstantInt(i32),
    ConstantBool(bool),
}
impl<'a> InlinedExpression<'a> {
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
        coll: &mut InlinedCollection<'a>,
    ) -> InlinedExpression<'a> {
        let hash = expr.get_hash();
        let output = match expr {
            &HashedExpression::ConstantValue(Literal::EnvirBool(ref envir_name), _) => {
                let b = ::std::env::vars()
                    .filter(|(name,_)| envir_name == name)
                    .flat_map(|(_,var)| bool::from_str(&var).ok())
                    .next()
                    .expect(&format!("could not fine value {} in environment", envir_name));
                InlinedExpression::ConstantBool(b)
            },
            &HashedExpression::ConstantValue(Literal::EnvirNumber(ref envir_name), _) => {
                let i = ::std::env::vars()
                    .filter(|(name,_)| envir_name == name)
                    .flat_map(|(_,var)| i32::from_str(&var).ok())
                    .next()
                    .expect(&format!("could not fine value {} in environment", envir_name));
                InlinedExpression::ConstantInt(i)
            },
            &HashedExpression::ConstantValue(Literal::Number(i), _) => {
                InlinedExpression::ConstantInt(i as i32)
            },
            &HashedExpression::ConstantValue(Literal::Boolean(b),_) => {
                InlinedExpression::ConstantBool(b)
            },
            &HashedExpression::ExternalConstant(ref id, _) | &HashedExpression::Var(ref id, _) => {
                // resolve the expression that defines the variable
                // convert that recursively
                InlinedExpression::variable(id, stack, coll)
            }
            &HashedExpression::FunctionArg(_, ref index, _) => {
                // compose a function argument into
                InlinedExpression::func_arg(index, stack, coll)
            }
            &HashedExpression::Func(ref id, ref args, _) => {
                InlinedExpression::func(id, args.as_ref(), &hash, stack, coll)
            }
            &HashedExpression::Op(ref left, op, ref right, out) => {
                // convert arguments into new format
                let left = InlinedExpression::new(stack.get_expr(left).unwrap(), stack, coll);
                let right = InlinedExpression::new(stack.get_expr(right).unwrap(), stack, coll);
                match (left,right) {
                    (InlinedExpression::ConstantBool(l),InlinedExpression::ConstantBool(r)) => {
                        match (out, op) {
                            (TypeData::Bool, Operation::And) => {
                                InlinedExpression::ConstantBool(l & r) 
                            },
                            (TypeData::Bool,Operation::Or) => {
                                InlinedExpression::ConstantBool(l | r)
                            },
                            _ => panic!("other boolean expressions are not possible"),
                        }
                    }
                    (InlinedExpression::ConstantInt(left),InlinedExpression::ConstantInt(right)) => {
                        match (out, op) {
                            (TypeData::Int, Operation::Add) => {
                                InlinedExpression::ConstantInt(left + right)
                            },
                            (TypeData::Int, Operation::Sub) => {
                                InlinedExpression::ConstantInt(left - right)
                            },
                            (TypeData::Int, Operation::Mul) => {
                                InlinedExpression::ConstantInt(left * right)
                            },
                            (TypeData::Int, Operation::Div) => {
                                InlinedExpression::ConstantInt(left / right)
                            },
                            (TypeData::Int, Operation::Or) => {
                                InlinedExpression::ConstantInt(left | right)
                            },
                            (TypeData::Int, Operation::And) => {
                                InlinedExpression::ConstantInt(left & right)
                            },
                            (TypeData::Bool, Operation::Equal) => {
                                InlinedExpression::ConstantBool(left == right)
                            },
                            (TypeData::Bool, Operation::GreaterThan) => {
                                InlinedExpression::ConstantBool(left > right)
                            },
                            (TypeData::Bool, Operation::LessThan) => {
                                InlinedExpression::ConstantBool(left < right)
                            },
                            (TypeData::Bool, Operation::GreaterThanEqual) => {
                                InlinedExpression::ConstantBool(left >= right)
                            },
                            (TypeData::Bool, Operation::LessThanEqual) => {
                                InlinedExpression::ConstantBool(left <= right)
                            },
                            _ => panic!("illegal interger operation"),
                        }
                    }
                    (left,right) => {
                        InlinedExpression::Operation(left.get_hash(), op, right.get_hash(), out)
                    },
                    anything_else => _unreachable_panic!("illegal operation. Should be caught by type checker. {:?}", anything_else)
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
        coll: &mut InlinedCollection<'a>,
    ) -> InlinedExpression<'a> {
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
        coll: &mut InlinedCollection<'a>,
    ) -> InlinedExpression<'a> {
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
        coll: &mut InlinedCollection<'a>,
    ) -> InlinedExpression<'a> {
        if stack.is_stdlib(id) {
            let mut new_args = Vec::<u64>::with_capacity(args.len());
            for arg in args.iter() {
                let expr = match stack.get_expr(arg) {
                    Option::None => _unreachable_panic!(),
                    Option::Some(ref expr) => expr.clone(),
                };
                let expr = InlinedExpression::new(expr, stack, coll);
                new_args.push(expr.get_hash());
            }
            let new_args = new_args.into_boxed_slice();
            let name = stack.get_function_name(id).unwrap();
            InlinedExpression::StdLibFunc(name, new_args)
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
