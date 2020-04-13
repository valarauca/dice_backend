use std::collections::BTreeMap;
use std::env::vars as envvars;
use std::str::FromStr;
use std::sync::Arc;

use super::super::rayon::ThreadPool;

use super::super::cfgbuilder::{ExpressionCollection, HashedExpression, Identifier};
use super::super::parser_output::Literal;
use super::{Data, ProbabilityDataType, ProbabilityFuture, Stack, TupleElement};

#[derive(Default)]
pub struct FutureTracker {
    data: BTreeMap<u64, ProbabilityFuture>,
    vars: BTreeMap<Identifier, u64>,
}
impl FutureTracker {
    /*
    pub fn new(coll: &ExpressionCollection) -> Self
    {
        let mut item = Self::default();

        // get the initial expression
        let ret = coll.get_return().unwrap();
        //item.build(coll, ret);

        item
    }
    */

    fn get_expr(
        &mut self,
        coll: &ExpressionCollection,
        stack: &mut Stack,
        expr: u64,
    ) -> ProbabilityFuture {
        let expr = coll.get_expr(stack.get_current_frame(), &expr).unwrap();
        self.build(coll, stack, expr)
    }

    fn build(
        &mut self,
        coll: &ExpressionCollection,
        stack: &mut Stack,
        expr: &HashedExpression,
    ) -> ProbabilityFuture {
        let hash = expr.get_hash();

        // actually resolve the expression
        match expr {
            &HashedExpression::Func(ref ident, ref args, _) => {
                match coll.get_function_name(ident).unwrap() {
                    "roll_d6" => {
                        let parent = self.get_expr(coll, stack, args[0]);
                        ProbabilityFuture::lambda(move || parent.get_data().rolld6())
                    }
                    "roll_d3" => {
                        let parent = self.get_expr(coll, stack, args[0]);
                        ProbabilityFuture::lambda(move || parent.get_data().rolld3())
                    }
                    "count" | "sum" => {
                        let parent = self.get_expr(coll, stack, args[0]);
                        ProbabilityFuture::lambda(move || parent.get_data().sum())
                    }
                    "filter" => {
                        let left = self.get_expr(coll, stack, args[0]);
                        let right = self.get_expr(coll, stack, args[1]);
                        ProbabilityFuture::lambda(move || left.get_data().filter(right.get_data()))
                    }
                    _ => {
                        // need to do weird function context changes
                        panic!()
                    }
                }
            }
            &HashedExpression::ConstantValue(ref lit, _) => {
                let future = match lit {
                    Literal::Boolean(ref b) => {
                        ProbabilityFuture::constant(TupleElement::constant_bool(b.clone()))
                    }
                    Literal::Number(ref b) => {
                        ProbabilityFuture::constant(TupleElement::constant_int(b.clone() as i32))
                    }
                    Literal::EnvirBool(ref name) => {
                        let envir = envvars()
                            .filter(|(key, _)| key == name)
                            .filter_map(|(_, value)| bool::from_str(&value).ok())
                            .next();
                        match envir {
                            Option::None => {
                                panic!("could not find true/faluse for name:'{}'", name);
                            }
                            Option::Some(b) => {
                                ProbabilityFuture::constant(TupleElement::constant_bool(b))
                            }
                        }
                    }
                    Literal::EnvirNumber(ref name) => {
                        let envir = envvars()
                            .filter(|(key, _)| key == name)
                            .filter_map(|(_, value)| i32::from_str_radix(&value, 10).ok())
                            .next();
                        match envir {
                            Option::None => {
                                panic!("could not find i32 for name:'{}'", name);
                            }
                            Option::Some(i) => {
                                ProbabilityFuture::constant(TupleElement::constant_int(i))
                            }
                        }
                    }
                };
                self.insert_future(hash, &future);
                future
            }
            &HashedExpression::Op(ref left_expr, ref op, ref right_expr, _) => {
                let left_expr = self.get_expr(coll, stack, left_expr);
                let right_expr = self.get_expr(coll, stack, right_expr);
            }
            &HashedExpression::ExternalConstant(ref ident, _) => panic!(),
            &HashedExpression::Var(ref ident, _) => panic!(),
            &HashedExpression::FunctionArg(_, _, _) => panic!(),
        }
    }

    fn insert_future(&mut self, hash: u64, future: &ProbabilityFuture) {
        self.data.insert(hash, future.clone());
    }
}
