
use super::super::parser_output::{Operation,TypeData};
use super::super::ordering::*;


pub enum Datum {
    Bool(bool),
    Int(i32),
    CollectionOfBool(Vec<bool>),
    CollectionOfInt(Vec<i32>),
}

pub struct Event {
    pub datum: Datum,
    pub prob: f64,
}
impl Event {
    pub fn new_int(x: i32) -> Event {
        Event {
            datum: Datum::Int(x),
            prob: 1.0f64,
        }
    }
    pub fn new_bool(x: bool) -> Event {
        Event {
            datum: Datum::Bool(x),
            prob: 1.0f64,
        }
    }
}

pub struct Prob {
    pub data: Vec<Event>,
}
impl From<Event> for Prob {
    fn from(e: Event) -> Prob {
        Prob {
            data: vec![e],
        }
    }
}

pub enum PartialApp {
    Zero(Box<dyn Fn() -> Prob>),
    One(Box<dyn Fn(Prob) -> Prob>),
    Two(Box<dyn Fn(Prob,Prob) -> Prob>),
    Three(Box<dyn Fn(Prob,Prob,Prob) -> Prob>)
}
impl PartialApp {
    fn new_zero<F>(lambda: F) -> Self
    where F: Fn() -> Prob + 'static
    {
        PartialApp::Zero(Box::new(lambda))
    }
}

pub fn partial_application<'a>(
    coll: &OrderingCollection<'a>,
    expr: &OrderedExpression<'a>) -> PartialApp {
    match expr {
        &OrderedExpression::ConstantBool(ref b) => {
            let b = b.lit.clone();
            PartialApp::new_zero(move || -> Prob {
                Prob::from(Event::new_bool(b))
            })
        },
        &OrderedExpression::ConstantInt(ref i) => {
            let i = i.lit.clone();
            PartialApp::new_zero(move || -> Prob {
                Prob::from(Event::new_int(i))
            })
        },
        &OrderedExpression::Op(ref op) => {
            let left = coll.get_expr(&op.left);
            let right = coll.get_expr(&op.right);
            let kind = op.kind.clone();
            match op.op {
                Operation::Sub | Operation::Mul | Operation::Div | Operation::Add => {
                    match op.kind {
                        TypeData::CollectionOfInt => {
                        }
                    }
                }
            }
        }
        _ => {
            panic!()
        }
    }
}
