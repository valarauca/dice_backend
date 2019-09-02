
//! credit to: https://pp.ipd.kit.edu/uploads/publikationen/braun13cc.pdf
//! 
//! but as an uninformed observer by querying the call chain lazily you are
//! just building a CFG, just an emergent one of causality.

use std::collections::{BTreeMap,HashSet,HashMap};
use std::hash::{Hasher,Hash};

use super::parser_output::*;
use super::seahasher::{DefaultSeaHasher};
use super::seahash::{SeaHasher};

pub struct Analysis<'a> {
    pub constants: HashMap<&'a str, ConstantDeclaration<'a>, DefaultSeaHasher>,
    pub functions: HashMap<&'a str, FunctionDeclaration<'a>, DefaultSeaHasher>,
}
impl<'a> Analysis<'a> {
    fn add_const<'b>(&mut self, arg: &'b Structures<'a>) -> Option<Result<(),String>> {
        let lambda = | arg: &'b ConstantDeclaration<'a>| -> Result<(),String> {
            // check functions first b/c no side effects
            match self.functions.get(arg.name) {
                Option::Some(is_a_func) => return Err(format!("constant named=\"{}\" cannot be declared, function=\"{}\" uses that name", is_a_func.name, is_a_func.name)),
                Option::None => { },
            };
            match self.constants.insert(arg.name, arg.clone()) {
                Option::Some(already_exists) => return Err(format!("const named=\"{}\" already exists", already_exists.name)),
                Option::None => { },
            };
            Ok(())
        };
        Structures::to_const(arg).into_iter().map(lambda).next()
    }
    fn add_function<'b>(&mut self, arg: &'b Structures<'a>) -> Option<Result<(),String>> {
        let lambda = | arg: &'b FunctionDeclaration<'a>| -> Result<(),String> {
            match self.constants.get(arg.name) {
                Option::Some(is_a_const) => return Err(format!("function named=\"{}\" cannot be declared, constant=\"{}\" uses that name", is_a_const.name, is_a_const.name)),
                Option::None => { },
            };
            match self.functions.insert(arg.name, arg.clone()) {
                Option::Some(already_exists) => return Err(format!("function named=\"{}\" already exists", already_exists.name)),
                Option::None => { },
            };
            Ok(())
        };
        Structures::to_func(arg).into_iter().map(lambda).next()
    }
}

/// BlockExpression is the result of expresion lowering.
/// when preformed it. Block Expressions, unlike normal
/// expressions are not a recrusive data type.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum BlockExpression<'a> {
    Constant(Literal<'a>,Option<TypeData>),
    Func(&'a str, Box<[u64]>,Option<TypeData>),
    Var(&'a str,Option<TypeData>),
    Op(u64, Operation, u64, Option<TypeData>),
}
impl<'a> BlockExpression<'a> {

    fn get_hash(&self) -> u64 {
        let mut seahash = SeaHasher::default();
        self.hash(&mut seahash);
        seahash.finish() 
    }
}

/// BasicBlock is in essence a function's body.
/// It can also be used for control structures
/// but presently the language has no control
/// structures.
///
/// This is an intermediate step to producing
/// a "real" SSA.
pub struct BasicBlock<'a> {
     expressions: BTreeMap<u64,BlockExpression<'a>>,
     vars: BTreeMap<u64,(&'a str, Option<TypeData>)>,
}
impl<'a> BasicBlock<'a> {
    /*
    pub fn new(statements: Statements<'a>) -> Result<BasicBlock<'a>,String> {
        BasicBlock::check_duplicate_vars(statements.data.into_iter())?;
    }
    */

    fn check_duplicate_vars<'b>(statements: &'b Statements<'a>) -> Result<(),String> {
        let mut map: HashSet<&'b str, DefaultSeaHasher>  = HashSet::default();
        for var in statements.data.iter().filter_map(Statement::get_variable_declaration) {
            if map.insert(var.name) {
                return Err(format!("variable named {} is used twice", var.name));
            }
        }
        Ok(())
    }

    /// add_expression will add an expression to the internal collection.
    /// if the expression already exists, it will return the ID to that 
    /// existing expression
    fn add_expression<R: AsRef<Expression<'a>>>(&mut self, expr: &R) -> u64 {
        let block_expr = match expr.as_ref() {
            Expression::Func(func) => {
                let mut args = func.args.iter().map(|arg| self.add_expression(arg)).collect::<Vec<u64>>().into_boxed_slice();
		BlockExpression::Func(func.name, args, None)
            },
            Expression::Variable(var_ref) => {
                BlockExpression::Var(var_ref.name, None)
            },
            Expression::Literal(lit_val) => {
                BlockExpression::Constant(lit_val.lit.clone(), Some(lit_val.lit.get_type()))
            },
            Expression::Operation(op) => {
                let left = self.add_expression(&op.left);
                let right = self.add_expression(&op.right); 
                BlockExpression::Op(left, op.op, right, None)
            }
        };
        let hash_value = block_expr.get_hash();
        self.expressions.insert(hash_value, block_expr);
        hash_value
    }
}
