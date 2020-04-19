use std::collections::BTreeMap;
use std::mem::replace;

use super::super::inliner::{InlinedCollection, InlinedExpression};
use super::super::parser_output::{Operation, TypeData};

use super::*;

#[derive(Default)]
pub struct Resolved<'a, 'b: 'a> {
    pub data: BTreeMap<&'a InlinedExpression<'b>, LambdaKind>,
}
impl<'a, 'b: 'a> Resolved<'a, 'b> {
    fn mark_idempotent(&mut self, expr: &&'a InlinedExpression<'b>) {
        match self.data.get_mut(expr) {
            Option::Some(ref mut lambda_kind) => {
                lambda_kind.make_idempotent();
            }
            _ => {}
        };
    }

    fn contains(&self, expr: &&'a InlinedExpression<'b>) -> bool {
        self.data.get(expr).is_some()
    }

    fn insert(&mut self, expr: &'a InlinedExpression<'b>, lambda: LambdaKind) {
        self.data.insert(expr, lambda);
    }

    fn resolve(&mut self, expr: &&'a InlinedExpression<'b>, args: &mut Vec<Iter>) -> Iter {
        match self.data.get_mut(expr) {
            Option::Some(ref mut lambda_kind) => lambda_kind.get_iter(args),
            Option::None => _unreachable_panic!(),
        }
    }
}

fn builder_recursive<'a, 'b: 'a>(
    resolve: &mut Resolved<'a, 'b>,
    coll: &'a InlinedCollection<'b>,
    expr: &'a InlinedExpression<'b>,
    stack: &mut Vec<Iter>,
) -> Iter {
    match expr {
        &InlinedExpression::ConstantBool(_) => resolve.resolve(&expr, stack),
        &InlinedExpression::ConstantInt(ref i) => resolve.resolve(&expr, stack),
        &InlinedExpression::StdLibFunc("roll_d6", ref args) => {
            debug_assert_eq!(args.len(), 1);
            let arg_iter =
                builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::StdLibFunc("roll_d3", ref args) => {
            debug_assert_eq!(args.len(), 1);
            let arg_iter =
                builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::StdLibFunc("count", ref args) => {
            debug_assert_eq!(args.len(), 1);
            let arg_iter =
                builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::StdLibFunc("len", ref args) => {
            debug_assert_eq!(args.len(), 1);
            let arg_iter =
                builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::StdLibFunc("join", ref args) => {
            debug_assert_eq!(args.len(), 2);
            let arg_iter2 =
                builder_recursive(resolve, coll, coll.get_expr(&args[1]).unwrap(), stack);
            stack.push(arg_iter2);
            let arg_iter1 =
                builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap(), stack);
            stack.push(arg_iter1);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::StdLibFunc("sum", ref args) => {
            debug_assert_eq!(args.len(), 1);
            let arg_iter =
                builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::StdLibFunc("filter", ref args) => {
            debug_assert_eq!(args.len(), 2);
            let arg_iter2 =
                builder_recursive(resolve, coll, coll.get_expr(&args[1]).unwrap(), stack);
            stack.push(arg_iter2);
            let arg_iter1 =
                builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap(), stack);
            stack.push(arg_iter1);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::StdLibFunc(ref name, _) => {
            panic!("invalid standard library function named {:?}", name);
        }
        &InlinedExpression::Operation(ref left, Operation::Add, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Add, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Add,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Add,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::Sub, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Sub, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Sub,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Sub,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::Mul, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Mul, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Mul,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Mul,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::Div, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Div, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Div,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Div,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::Or, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Or, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Or,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Or,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::And, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::And, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::And,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::And,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::Equal, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Equal, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Equal,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Equal,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThan,
            ref right,
            TypeData::Int,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThan,
            ref right,
            TypeData::Bool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThan,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThan,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::LessThan, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::LessThan, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThan,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThan,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThanEqual,
            ref right,
            TypeData::Int,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThanEqual,
            ref right,
            TypeData::Bool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThanEqual,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThanEqual,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThanEqual,
            ref right,
            TypeData::Int,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThanEqual,
            ref right,
            TypeData::Bool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThanEqual,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThanEqual,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::NotEqual, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::NotEqual, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::NotEqual,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::NotEqual,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
    }
}

/// shoves thing into our resolve structure
fn lambda_builder_recursive<'a, 'b: 'a>(
    resolve: &mut Resolved<'a, 'b>,
    coll: &'a InlinedCollection<'b>,
    expr: &'a InlinedExpression<'b>,
) {
    if resolve.contains(&expr) {
        // updates collection so value is idempotent
        resolve.mark_idempotent(&expr);
        return;
    }
    match expr {
        &InlinedExpression::ConstantBool(ref b) => {
            resolve.insert(expr, LambdaKind::Init(const_bool(b.clone())))
        }
        &InlinedExpression::ConstantInt(ref i) => {
            resolve.insert(expr, LambdaKind::Init(const_int(i.clone())))
        }
        &InlinedExpression::StdLibFunc("roll_d6", ref args) => {
            debug_assert_eq!(args.len(), 1);
            // build the argument
            lambda_builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap());
            // insert self
            resolve.insert(expr, LambdaKind::Chain(d6()));
        }
        &InlinedExpression::StdLibFunc("roll_d3", ref args) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap());
            resolve.insert(expr, LambdaKind::Chain(d3()));
        }
        &InlinedExpression::StdLibFunc("count", ref args) => {
            debug_assert_eq!(args.len(), 1);
            lambda_builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap());
            resolve.insert(expr, LambdaKind::Chain(count()));
        }
        &InlinedExpression::StdLibFunc("len", ref args) => {
            debug_assert_eq!(args.len(), 1);
            lambda_builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap());
            resolve.insert(expr, LambdaKind::Chain(len()));
        }
        &InlinedExpression::StdLibFunc("join", ref args) => {
            debug_assert_eq!(args.len(), 2);
            lambda_builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap());
            lambda_builder_recursive(resolve, coll, coll.get_expr(&args[1]).unwrap());
            resolve.insert(expr, LambdaKind::Combinator(join()));
        }
        &InlinedExpression::StdLibFunc("sum", ref args) => {
            debug_assert_eq!(args.len(), 1);
            lambda_builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap());
            resolve.insert(expr, LambdaKind::Chain(sum()));
        }
        &InlinedExpression::StdLibFunc("filter", ref args) => {
            debug_assert_eq!(args.len(), 2);
            lambda_builder_recursive(resolve, coll, coll.get_expr(&args[0]).unwrap());
            lambda_builder_recursive(resolve, coll, coll.get_expr(&args[1]).unwrap());
            resolve.insert(expr, LambdaKind::Combinator(filter()));
        }
        &InlinedExpression::StdLibFunc(ref name, _) => {
            panic!("invalid standard library function named {:?}", name);
        }
        &InlinedExpression::Operation(ref left, Operation::Add, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Add, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Add,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Add,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::Sub, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Sub, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Sub,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Sub,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::Mul, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Mul, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Mul,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Mul,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::Div, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Div, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Div,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Div,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::Or, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Or, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Or,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Or,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::And, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::And, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::And,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::And,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::Equal, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::Equal, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::Equal,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::Equal,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThan,
            ref right,
            TypeData::Int,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThan,
            ref right,
            TypeData::Bool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThan,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThan,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::LessThan, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::LessThan, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThan,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThan,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThanEqual,
            ref right,
            TypeData::Int,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThanEqual,
            ref right,
            TypeData::Bool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThanEqual,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::GreaterThanEqual,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThanEqual,
            ref right,
            TypeData::Int,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThanEqual,
            ref right,
            TypeData::Bool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThanEqual,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::LessThanEqual,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(ref left, Operation::NotEqual, ref right, TypeData::Int) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(ref left, Operation::NotEqual, ref right, TypeData::Bool) => {
            panic!("invalid expression")
        }
        &InlinedExpression::Operation(
            ref left,
            Operation::NotEqual,
            ref right,
            TypeData::CollectionOfInt,
        ) => panic!("invalid expression"),
        &InlinedExpression::Operation(
            ref left,
            Operation::NotEqual,
            ref right,
            TypeData::CollectionOfBool,
        ) => panic!("invalid expression"),
    }
}
