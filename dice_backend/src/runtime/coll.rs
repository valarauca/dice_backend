use std::collections::BTreeMap;
use std::mem::replace;

use super::super::ordering::{OrderedCollection, OrderedExpression, StdLibraryFunc, OrdTrait, ConstantValue};
use super::super::parser_output::{Operation, TypeData};

use super::*;

#[derive(Default)]
pub struct Resolved<'a> {
    pub data: BTreeMap<&'a OrderedExpression, LambdaKind>,
}
impl<'a> Resolved<'a> {
    fn mark_idempotent(&mut self, expr: &&'a OrderedExpression) {
        match self.data.get_mut(expr) {
            Option::Some(ref mut lambda_kind) => {
                lambda_kind.make_idempotent();
            }
            _ => {}
        };
    }

    fn contains(&self, expr: &&'a OrderedExpression) -> bool {
        self.data.get(expr).is_some()
    }

    fn insert(&mut self, expr: &'a OrderedExpression, lambda: LambdaKind) {
        self.data.insert(expr, lambda);
    }

    fn resolve(&mut self, expr: &&'a OrderedExpression, args: &mut Vec<Iter>) -> Iter {
        match self.data.get_mut(expr) {
            Option::Some(ref mut lambda_kind) => lambda_kind.get_iter(args),
            Option::None => _unreachable_panic!(),
        }
    }
}

/// runs the program & builds the report
pub fn build_report<'a>(coll: &'a OrderedCollection) -> Report {
    let mut resolved = Resolved::default();

    let ret = coll.get_expr(coll.get_return()).unwrap();
    lambda_builder_recursive(&mut resolved, coll, ret);

    let mut stack = Vec::<Iter>::new();
    builder_recursive(&mut resolved, coll, ret, &mut stack).collect()
}

// builds the stack of computations to run
fn builder_recursive<'a>(
    resolve: &mut Resolved<'a>,
    coll: &'a OrderedCollection,
    expr: &'a OrderedExpression,
    stack: &mut Vec<Iter>,
) -> Iter {

    let sources = expr.get_sources();
    match sources.len() {
        0 => {
            resolve.resolve(&expr, stack)
        }
        1 => {
            let arg = sources[0].0;
            let arg_iter = builder_recursive(resolve, coll, coll.get_expr(arg).unwrap(), stack);
            stack.push(arg_iter);
            resolve.resolve(&expr, stack)
        }
        2 => {
            let left = sources[0].0;
            let right = sources[1].0;
            let arg_left = builder_recursive(resolve, coll, coll.get_expr(left).unwrap(), stack);
            let arg_right = builder_recursive(resolve, coll, coll.get_expr(right).unwrap(), stack);
            stack.push(arg_right);
            stack.push(arg_left);
            resolve.resolve(&expr, stack)
        }
        _ => {
            unimplemented!("3 or more argument byte code isn't supported");
        }
    };
}

// converts enum's into boxed iterators that represent their
// calcs
fn lambda_builder_recursive<'a>(
    resolve: &mut Resolved<'a>,
    coll: &'a OrderedCollection,
    expr: &'a OrderedExpression,
) {
    if resolve.contains(&expr) {
        // updates collection so value is idempotent
        resolve.mark_idempotent(&expr);
        return;
    }
    match expr {
        &OrderedExpression::Constant(ConstantValue::Bool(ref b,_)) => {
            resolve.insert(expr, LambdaKind::Init(const_bool(b.clone())))
        },
        &OrderedExpression::Constant(ConstantValue::Int(ref i,_)) => {
            resolve.insert(expr, LambdaKind::Init(const_int(i.clone())))
        },
        &OrderedExpression::StdLib(ref std) => {
            let sources = std.get_sources();
            match sources.len() {
                1 => {
                    let arg = sources[0].0;
                    match std {
                        &StdLibraryFunc::D6(_) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(d3()));
                        }
                        &StdLibraryFunc::D3(_) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(d6()));
                        }
                        &StdLibraryFunc::Count(_) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(count()));
                        }
                        &StdLibraryFunc::Len(_) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(len()));
                        }
                        &StdLibraryFunc::Sum(_) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(sum()));
                        }
                        &StdLibraryFunc::Max(_) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(max()));
                        }
                        &StdLibraryFunc::Min(_) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(arg).unwrap());
            resolve.insert(expr, LambdaKind::Chain(min()));
                        }
                        x => {
                            panic!("{:?} should not have 1 argument", x);
                        }
                    };
                }
                2 => {
                    let a = sources[0].0;
                    let b = sources[1].0;
                    match std {
                        &StdLibraryFunc::Join(_) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(a).unwrap());
            lambda_builder_recursive(resolve, coll, coll.get_expr(b).unwrap());
            resolve.insert(expr, LambdaKind::Combinator(join()));
                        }
                        &StdLibraryFunc::Filter(_) => {
            lambda_builder_recursive(resolve, coll, coll.get_expr(a).unwrap());
            lambda_builder_recursive(resolve, coll, coll.get_expr(b).unwrap());
            resolve.insert(expr, LambdaKind::Combinator(filter()));
                        },
                        x => {
                            panic!("{:?} should  not have 2 arguments", x);
                        }
                    }
                }
            }
        }
        &OrderedExpression::Op(ref op) => {
            let sources = op.get_sources();
            debug_assert_eq!(sources.len(), 2);
            let left = sources[0].0;
            let right = sources[0].0;
            lambda_builder_recursive(resolve, coll, coll.get_expr(left).unwrap());
            lambda_builder_recursive(resolve, coll, coll.get_expr(right).unwrap());
            resolve.insert(expr, LambdaKind::Combinator(from_op(op)));
        }
        x => {
            panic!("{:?} is not implemented",x );
        }
    }
}
