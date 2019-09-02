
//! credit to: https://pp.ipd.kit.edu/uploads/publikationen/braun13cc.pdf
//! 
//! but as an uninformed observer by querying the call chain lazily you are
//! just building a CFG, just an emergent one of causality.

use std::collections::{BTreeMap,HashSet,HashMap};
use std::hash::{Hasher,Hash};

use super::parser_output::{Structures,TypeData};
use super::seahasher::{DefaultSeaHasher};
use super::seahash::{SeaHasher};


pub struct GlobalScope<'a> {
    functions_type_data: HashMap<&'a str, TypeData, DefaultSeaHasher>,    
    functions_arg_data: HashMap<&'a str, Box<[(&'a str, TypeData)]>,
    function_statement_data: HashMap<&'a str, Statements<'a>>
    constants_type_data: HashMap<&'a str, TypeData, DefaultSeaHasher>,
}
impl<'a> GlobalScope<'a> {

    fn gather_function_data<'b>(&mut self, structures: &'b [Structures<'a>]) -> Result<(),String> {
        for func_dec in structures.into_iter().filter_map(Structures::to_func) {
            match self.constants_type_data.get(func_dec.name) {
                Option::Some(_) => return Err(format!("function & constant are both named '{}' this is illegal", func_dec.name)),
                Option::None => { },
            };
            match self.functions_type_data.insert(func_dec.name, func_dec.ret) {
                Option::Some(_) => return Err(format!("multiple functions with name '{}' encountered", func_dec.name)),
                Option::None => { }
            };
            self.functions_arg_data.insert(func_dec.name, func_dec.args.clone());
        }
        Ok(())
    }

    fn gather_constant_type_data<'b>(&mut self, structures: &'b [Structures<'a>]) -> Result<(), String> {
        for cons in structures.into_iter().filter_map(Structures::to_const) {
            match self.functions_type_data.get(cons.name) {
                Option::Some(_) => return Err(format!("function & constant are both named '{}' this is illegal", cons.name)),
                Option::None => { },
            };
            match self.cons_type_data.insert(cons.name, cons.kind) {
                Option::Some(_) => return Err(format!("multiple constants with name '{}' encountered", cons.name)),
                Option::None => { }
            };
        }
        Ok(())
    }
    
    /*
     * General to avoid low-hanging-fruit syntax errors
     *
     */ 
    fn multiple_analyze<'b>(structures: &'b [Structures<'a>]) -> Result<(),String> {
        let analyze_statements = structures.iter().filter(|item| match item {
            Structures::Analyze(_) => true,
            _ => false,
        }).count();
        if analyze_statements > 1 {
            return Err(format!("There are only be 1 analyze statement per input"));
        }
        Ok(())
    }
}

/*
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
     vars: BTreeMap<u64,(&'a str,Option<TypeData>)>,
}
impl<'a> BasicBlock<'a> {
    /*
    pub fn new(statements: Statements<'a>) -> Result<BasicBlock<'a>,String> {
        BasicBlock::check_duplicate_vars(statements.data.into_iter())?;
    }
    */

/*
    fn check_duplicate_vars<'b>(statements: &'b Statements<'a>) -> Result<(),String> {
        let mut map: HashSet<&'b str, DefaultSeaHasher>  = HashSet::default();
        for var in statements.data.iter().filter_map(Statement::get_variable_declaration) {
            if map.insert(var.name) {
                return Err(format!("variable named {} is used twice", var.name));
            }
        }
        Ok(())
    }
*/

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
*/
