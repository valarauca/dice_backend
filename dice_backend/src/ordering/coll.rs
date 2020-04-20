use std::collections::BTreeMap;
use std::mem::replace;

use super::super::inliner::{InlinedCollection, InlinedExpression};
use super::super::parser_output::{Operation, TypeData};

use super::*;

#[derive(Default)]
pub struct Resolved<'a> {
    pub data: BTreeMap<&'a InlinedExpression, LambdaKind>,
}
impl<'a> Resolved<'a> {
    fn mark_idempotent(&mut self, expr: &&'a InlinedExpression) {
        match self.data.get_mut(expr) {
            Option::Some(ref mut lambda_kind) => {
                lambda_kind.make_idempotent();
            }
            _ => {}
        };
    }

    fn contains(&self, expr: &&'a InlinedExpression) -> bool {
        self.data.get(expr).is_some()
    }

    fn insert(&mut self, expr: &'a InlinedExpression, lambda: LambdaKind) {
        self.data.insert(expr, lambda);
    }

    fn resolve(&mut self, expr: &&'a InlinedExpression, args: &mut Vec<Iter>) -> Iter {
        match self.data.get_mut(expr) {
            Option::Some(ref mut lambda_kind) => lambda_kind.get_iter(args),
            Option::None => _unreachable_panic!(),
        }
    }
}

/// runs the program & builds the report
pub fn build_report<'a>(coll: &'a InlinedCollection) -> Report {
    let mut resolved = Resolved::default();

    let ret = coll
        .get_return()
        .into_iter()
        .flat_map(|expr| coll.get_expr(&expr))
        .next()
        .unwrap();
    lambda_builder_recursive(&mut resolved, coll, ret);

    let mut stack = Vec::<Iter>::new();
    builder_recursive(&mut resolved, coll, ret, &mut stack).collect()
}

fn builder_recursive<'a>(
    resolve: &mut Resolved<'a>,
    coll: &'a InlinedCollection,
    expr: &'a InlinedExpression,
    stack: &mut Vec<Iter>,
) -> Iter {
    match expr {
        &InlinedExpression::ConstantBool(_) => resolve.resolve(&expr, stack),
        &InlinedExpression::ConstantInt(ref i) => resolve.resolve(&expr, stack),
        &InlinedExpression::D6(ref arg) => {
            let arg_iter = builder_recursive(resolve, coll, coll.get_expr(arg).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::D3(ref arg) => {
            let arg_iter = builder_recursive(resolve, coll, coll.get_expr(arg).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::Count(ref arg) => {
            let arg_iter = builder_recursive(resolve, coll, coll.get_expr(arg).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::Len(ref arg) => {
            let arg_iter = builder_recursive(resolve, coll, coll.get_expr(arg).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::Join(ref a, ref b) => {
            let arg_iter2 = builder_recursive(resolve, coll, coll.get_expr(b).unwrap(), stack);
            let arg_iter1 = builder_recursive(resolve, coll, coll.get_expr(a).unwrap(), stack);
            stack.push(arg_iter2);
            stack.push(arg_iter1);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::Sum(ref arg) => {
            let arg_iter = builder_recursive(resolve, coll, coll.get_expr(arg).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        &InlinedExpression::Filter(ref a, ref b) => {
            let arg_iter2 = builder_recursive(resolve, coll, coll.get_expr(b).unwrap(), stack);
            let arg_iter1 = builder_recursive(resolve, coll, coll.get_expr(a).unwrap(), stack);
            stack.push(arg_iter2);
            stack.push(arg_iter1);
            resolve.resolve(&expr, stack)
        }
        _ => panic!(),
    }
}

/// shoves thing into our resolve structure
fn lambda_builder_recursive<'a>(
    resolve: &mut Resolved<'a>,
    coll: &'a InlinedCollection,
    expr: &'a InlinedExpression,
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
        &InlinedExpression::D6(ref arg) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(d6()));
        }
        &InlinedExpression::D3(ref arg) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(d3()));
        }
        &InlinedExpression::Count(ref arg) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(count()));
        }
        &InlinedExpression::Len(ref arg) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(len()));
        }
        &InlinedExpression::Join(ref a, ref b) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(a).unwrap());
            lambda_builder_recursive(resolve, coll, coll.get_expr(b).unwrap());
            resolve.insert(expr, LambdaKind::Combinator(join()));
        }
        &InlinedExpression::Sum(ref arg) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(sum()));
        }
        &InlinedExpression::Filter(ref a, ref b) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(a).unwrap());
            lambda_builder_recursive(resolve, coll, coll.get_expr(b).unwrap());
            resolve.insert(expr, LambdaKind::Combinator(filter()));
        }
        _ => panic!()
    }
}
