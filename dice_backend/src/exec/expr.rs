
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

pub struct PartialApp {
    arg: Box<dyn Fn(Box<dyn IntoIterator<Item=Event>>)->Box<dyn IntoIterator<Item=Event>>>
}

pub enum PartialApplication {
    Zero(Box<dyn Fn() -> Box<dyn IntoIterator<Item=Event>>>),
    One(Box<dyn Fn(Box<dyn IntoIterator<Item=Event>>) -> Box<dyn IntoIterator<Item=Event>>>)
}

pub fn partial_application<'a>(
    coll: &OrderingCollection<'a>,
    expr: &OrderedExpression<'a>) -> PartialApplication {
    match expr {
        &OrderedExpression::ConstantBool(ref b) => {
            PartialApplication::Zero(Box::new(move || -> Box<dyn IntoIterator<Item=Event>> {
                Box::new(Some(Event{
                    datum: Datum::Bool(b.clone()),
                    prob: 1.0
                }))
            })
        },
        &OrderedExpression::ConstantInt(ref i) => {
            PartialApplication::Zero(Box::new(move || -> Box<dyn IntoIterator<Item=Event>> {
                Box::new(Some(Event{
                    datum: Datum::Int(i.clone()),
                    prob: 1.0,
                }))
            })
        },
        _ => {
            panic!()
        }
    }
}
