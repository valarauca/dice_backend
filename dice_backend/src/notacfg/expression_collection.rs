

use super::super::super::parser_output::*;

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum BlockExpression<'a> {
    Constant{
      literal: Literal<'a>,
      kind: TypeData, 
    },
    Function{
      name: &'a str,
      args: Box<[u64]>,
      kind: TypeData,
    },
    Var{
      name: &'a str,
      kind: TypeData, 
    },
    Op{
      left: u64,
      op: Operation,
      right: u64,
    },
}
