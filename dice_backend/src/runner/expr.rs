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
        let hash = match coll.get_from_hashed(expr) {
            (hash, Option::Some(expr)) => return expr.clone(),
            (hash, Option::None) => hash,
        };
        let output = match expr {
            &HashedExpression::ConstantValue(Literal::EnvirBool(ref envir_name), _) => {
                let b = ::std::env::vars()
                    .filter(|(name,_)| envir_name == name)
                    .flat_map(|(_,var)| bool::from_str(&var).ok())
                    .next()
                    .expect(&format!("could not fine value {} in environment", envir_name));
                Some(InlinedExpression::ConstantBool(b))
            },
            &HashedExpression::ConstantValue(Literal::EnvirNumber(ref envir_name), _) => {
                let i = ::std::env::vars()
                    .filter(|(name,_)| envir_name == name)
                    .flat_map(|(_,var)| i32::from_str(&var).ok())
                    .next()
                    .expect(&format!("could not fine value {} in environment", envir_name));
                Some(InlinedExpression::ConstantInt(i))
            },
            &HashedExpression::ConstantValue(Literal::Number(i), _) => {
                Some(InlinedExpression::ConstantInt(i as i32))
            },
            &HashedExpression::ConstantValue(Literal::Boolean(b),_) => {
                Some(InlinedExpression::ConstantBool(b))
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
                match (stack.get_expr(left), stack.get_expr(right)) {
                    (Option::Some(&HashedExpression::ConstantValue(Literal::Boolean(ref left),TypeData::Bool)),Option::Some(&HashedExpression::ConstantValue(Literal::Boolean(ref right),TypeData::Bool))) => {
                        match (out, op) {
                            (TypeData::Bool, Operation::And) => {
                                Some(InlinedExpression::ConstantBool(left & right))
                            },
                            (TypeData::Bool, Operation::Or) => {
                                Some(InlinedExpression::ConstantBool(left | right))
                             },
                             anything_else => _unreachable_panic!("illegal operation with boolean values. Should be caught by type checker. {:?}", anything_else)
                        }
                    }
                    (Option::Some(&HashedExpression::ConstantValue(Literal::Number(ref left),TypeData::Int)),Option::Some(&HashedExpression::ConstantValue(Literal::Number(ref right),TypeData::Int))) => {
                        match (out, op) {
                         (TypeData::Int, Operation::Add) => {
                             Some(InlinedExpression::ConstantInt(*left as i32 + *right as i32))
                         },
                         (TypeData::Int, Operation::Sub) => {
                             Some(InlinedExpression::ConstantInt(*left as i32 - *right as i32))
                         },
                         (TypeData::Int, Operation::Mul) => {
                             Some(InlinedExpression::ConstantInt(*left as i32 * *right as i32))
                         },
                         (TypeData::Int, Operation::Div) => {
                             Some(InlinedExpression::ConstantInt(*left as i32 / *right as i32))
                         },
                         (TypeData::Int, Operation::Or) => {
                             Some(InlinedExpression::ConstantInt(*left as i32 | *right as i32))
                         },
                         (TypeData::Int, Operation::And) => {
                             Some(InlinedExpression::ConstantInt(*left as i32 & *right as i32))
                         },
                         (TypeData::Bool, Operation::Equal) => {
                             Some(InlinedExpression::ConstantBool(*left == *right))
                         },
                         (TypeData::Bool, Operation::GreaterThan) => {
                             Some(InlinedExpression::ConstantBool(*left > *right))
                         },
                         (TypeData::Bool, Operation::LessThan) => {
                             Some(InlinedExpression::ConstantBool(*left < *right))
                         },
                         (TypeData::Bool, Operation::GreaterThanEqual) => {
                             Some(InlinedExpression::ConstantBool(*left >= *right))
                         },
                         (TypeData::Bool, Operation::LessThanEqual) => {
                             Some(InlinedExpression::ConstantBool(*left <= *right))
                         },
                         anything_else => _unreachable_panic!("illegal operation with interger constants. Should be caught by type checker. {:?}", anything_else)
                        }
                    },
                    (Option::Some(ref left), Option::Some(ref right)) => {
                        let left = InlinedExpression::new(left, stack, coll).get_hash();
                        let right = InlinedExpression::new(right, stack, coll).get_hash();
                        Some(InlinedExpression::Operation(left, op, right, out))
                    },
                    anything_else => _unreachable_panic!("illegal operation. Should be caught by type checker. {:?}", anything_else)
                }
            }
        };
        match &output {
            &Option::Some(ref out) => {
                // insert the new expresion into our pool
                // this ensure the next time we call `InlinedExpression::new`
                // if our `HashedExpression` was already encountered we won't
                // do massive amounts of pattern matching, and potentially
                // deep recursion.
                coll.insert_hash(&hash, out);
            }
            &Option::None => {
                // debug assertions are just a lazy man's tests right?
                _unreachable_panic!("no input for {:?}", expr)
            }
        };
        output.unwrap()
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
    ) -> Option<InlinedExpression<'a>> {
        stack
            .get_var(id)
            .into_iter()
            .map(|expr| InlinedExpression::new(expr, stack, coll))
            .next()
    }

    #[inline(always)]
    fn func_arg<'b>(
        arg_index: &usize,
        stack: &mut CallStack<'a, 'b>,
        coll: &mut InlinedCollection<'a>,
    ) -> Option<InlinedExpression<'a>> {
        let context = stack.get_context().unwrap();
        let func_expr = stack.get_ctx_expr().unwrap();
        let arg_expr = stack.get_arg_index(*arg_index).unwrap();
        stack.pop();
        let out = InlinedExpression::new(stack.get_expr(&arg_expr).unwrap(), stack, coll);
        stack.push(&context, &func_expr);
        Some(out)
    }

    #[inline(always)]
    fn func<'b>(
        id: &Identifier,
        args: &[u64],
        hash: &u64,
        stack: &mut CallStack<'a, 'b>,
        coll: &mut InlinedCollection<'a>,
    ) -> Option<InlinedExpression<'a>> {
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
            Some(InlinedExpression::StdLibFunc(name, new_args))
        } else {
            stack.push(id, &hash);
            let output = stack
                .get_return()
                .into_iter()
                .map(|expr| InlinedExpression::new(expr, stack, coll))
                .next();
            stack.pop();
            output
        }
    }
}
