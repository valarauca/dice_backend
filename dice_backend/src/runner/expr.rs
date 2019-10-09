use std::hash::{Hash, Hasher};

use super::super::cfgbuilder::{CallStack, ExpressionCollection, HashedExpression, Identifier};
use super::super::parser_output::{Literal, Operation, TypeData};
use super::super::seahash::SeaHasher;

use super::coll::InlinedCollection;

/// Inlined Expression contains the very base values
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InlinedExpression<'a> {
    StdLibFunc(&'a str, Box<[u64]>),
    Operation(
        Box<InlinedExpression<'a>>,
        Operation,
        Box<InlinedExpression<'a>>,
    ),
    Constant(Literal<'a>),
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
            &HashedExpression::ConstantValue(ref literal, _) => {
                Some(InlinedExpression::Constant(literal.clone()))
            }
            &HashedExpression::ExternalConstant(ref id, _) | &HashedExpression::Var(ref id, _) => {
                // resolve the expression that defines the variable
                // convert that recursively
                stack
                    .get_var(id)
                    .into_iter()
                    .map(|expr| InlinedExpression::new(expr, stack, coll))
                    .next()
            }
            &HashedExpression::FunctionArg(ref name, ref index, _) => {
                let context = stack.get_context().unwrap();
                let func_expr = stack.get_ctx_expr().unwrap();
                let arg_expr = stack.get_arg_index(index.clone()).unwrap();
                stack.pop();
                let output =
                    InlinedExpression::new(stack.get_expr(&arg_expr).unwrap(), stack, coll);
                stack.push(&context, &func_expr);
                Some(output)
            }
            &HashedExpression::Func(ref id, ref args, _) => {
                if stack.is_stdlib(id) {
                    let mut new_args = Vec::<u64>::with_capacity(args.len());
                    for arg in args.iter() {
                        let expr = match stack.get_expr(arg) {
                            Option::None => unreachable!(),
                            Option::Some(ref expr) => expr.clone()
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
            &HashedExpression::Op(ref left, op, ref right, out) => {
                match (stack.get_expr(left), stack.get_expr(right)) {
                    (Option::Some(&HashedExpression::ConstantValue(Literal::Boolean(ref left),TypeData::Bool)),Option::Some(&HashedExpression::ConstantValue(Literal::Boolean(ref right),TypeData::Bool))) => {
                        match (out, op) {
                            (TypeData::Bool, Operation::And) => {
                                Some(InlinedExpression::Constant(Literal::Boolean(left & right)))
                             },
                             (TypeData::Bool, Operation::Or) => {
                                Some(InlinedExpression::Constant(Literal::Boolean(left | right)))
                             },
                             anything_else => panic!("illegal operation with boolean values. Should be caught by type checker. {:?}", anything_else)
                        }
                    }
                    (Option::Some(&HashedExpression::ConstantValue(Literal::Number(ref left),TypeData::Int)),Option::Some(&HashedExpression::ConstantValue(Literal::Number(ref right),TypeData::Int))) => {
                        match (out, op) {
                         (TypeData::Int, Operation::Add) => {
                             Some(InlinedExpression::Constant(Literal::Number(left + right)))
                         },
                         (TypeData::Int, Operation::Sub) => {
                             Some(InlinedExpression::Constant(Literal::Number(left - right)))
                         },
                         (TypeData::Int, Operation::Mul) => {
                             Some(InlinedExpression::Constant(Literal::Number(left * right)))
                         },
                         (TypeData::Int, Operation::Div) => {
                             Some(InlinedExpression::Constant(Literal::Number(left / right)))
                         },
                         (TypeData::Int, Operation::Or) => {
                             Some(InlinedExpression::Constant(Literal::Number(left | right)))
                         },
                         (TypeData::Int, Operation::And) => {
                             Some(InlinedExpression::Constant(Literal::Number(left & right)))
                         },
                         (TypeData::Bool, Operation::Equal) => {
                             Some(InlinedExpression::Constant(Literal::Boolean(left == right)))
                         },
                         (TypeData::Bool, Operation::GreaterThan) => {
                             Some(InlinedExpression::Constant(Literal::Boolean(left > right)))
                         },
                         (TypeData::Bool, Operation::LessThan) => {
                             Some(InlinedExpression::Constant(Literal::Boolean(left < right)))
                         },
                         (TypeData::Bool, Operation::GreaterThanEqual) => {
                             Some(InlinedExpression::Constant(Literal::Boolean(left >= right)))
                         },
                         (TypeData::Bool, Operation::LessThanEqual) => {
                             Some(InlinedExpression::Constant(Literal::Boolean(left <= right)))
                         },
                         anything_else => panic!("illegal operation with interger constants. Should be caught by type checker. {:?}", anything_else)
                        }
                    },
                    anything_else => panic!("not supported"),
                }
            }
        };
        match &output {
            &Option::Some(ref out) => {
                coll.insert_hash(&hash, out);
            }
            &Option::None => panic!("no input for {:?}", expr),
        };
        output.unwrap()
    }
}
