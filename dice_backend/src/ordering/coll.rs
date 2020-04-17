
use super::super::runner::{InlinedCollection,InlinedExpression};
use super::super::parser_output::{Operation, TypeData};

use super::{Element,Datum};

type Iter = Box<dyn Iterator<Item=Element>>;
type Unary = Box<dyn Fn() -> Iter>;
type Filter = Box<dyn Fn(Element) -> Vec<Element>>;
type Modified = Box<dyn Fn(Option<Filter>) -> Iter>;

enum PartialFunction {
    Iter(Iter),
    Unary(Unary),
    Filter(Filter),
    Modified(Modified)
}

/*
pub fn lambda_builder<'a>(coll: &InlinedCollection<'a>) -> Unary {

    let return_expr = coll
        .get_return()
        .into_iter()
        .filter_map(|ret_id| arg.get_expr(ret_id))
        .next()
        .unwrap();
    panic!()
}
*/

fn lambda_builder_recursive<'a>(
    coll: &InlinedCollection<'a>,
    expr: &InlinedExpression<'a>
) -> Unary {
    match expr {
        &InlinedExpression::ConstantBool(ref b) => {
            let boolean_value: bool = b.clone();
            Box::new(move || -> Iter {
                Element::new(boolean_value, 1.0).build_iter()
            })
        },
        &InlinedExpression::ConstantInt(ref i) => {
            let int_value: i32 = i.clone();
            Box::new(move || -> Iter {
                Element::new(int_value, 1.0).build_iter()
            })
        },
        &InlinedExpression::Operation(ref l, Operation::Add, ref r, TypeData::Int) => {
        }
        _ => {
            panic!()
        }
    }
}
