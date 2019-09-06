
use std::collections::{HashMap};
use std::hash::{Hash,Hasher};
use std::mem::{replace};

use super::super::parser_output::{VariableDeclaration,TerminalExpression,Expression,TypeData,GetType,Statement,Statements};
use super::super::seahasher::{DefaultSeaHasher};

use super::namespace::{Namespace};
use super::blockexpression::{BlockExpression};


/// BasicBlock is in essence a function's body.
/// It can also be used for control structures
/// but presently the language has no control
/// structures.
///
/// This is an intermediate step to producing
/// a "real" SSA.
pub struct BasicBlock<'a, 'b> {
    namespace: &'b Namespace<'a>,
    vars: HashMap<&'a str, VariableDeclaration<'a>, DefaultSeaHasher>,
    populated_vars: HashMap<&'a str, BlockExpression<'a>, DefaultSeaHasher>,
    return_expression: Option<TerminalExpression<'a>>,
    populated_return_expresion: Option<BlockExpression<'a>>,
}
impl<'a, 'b> BasicBlock<'a, 'b> {
    pub fn new(
        external_names: &'b Namespace<'a>,
        statements: Statements<'a>,
    ) -> Result<BasicBlock<'a, 'b>, String> {
        let mut bb = BasicBlock {
            namespace: external_names,
            vars: HashMap::default(),
            populated_vars: HashMap::default(),
            return_expression: None,
            populated_return_expresion: None,
        };
        for stmt in statements.data.iter() {
            bb.add_statement(stmt)?;
        }
        if bb.return_expression.is_none() {
            // TODO bad error
            return Err(format!("function has no return statement"));
        }
        Ok(bb)
    }

    fn add_statement<'c>(&mut self, stmt: &'c Statement<'a>) -> Result<(), String> {
        match stmt {
            Statement::Variable(ref var) => {
                if self.namespace.is_name_defined(var.name) {
                    return Err(format!(
                        "variable:'{}' defined in\n{}\ncollides with function/constant name",
                        var.name, stmt
                    ));
                } else {
                    match self.vars.insert(var.name, var.clone()) {
                        Option::None => {}
                        Option::Some(_) => {
                            return Err(format!(
                                "variable of name '{}' is defined multiple times",
                                var.name
                            ))
                        }
                    };
                    self.populated_vars
                        .insert(var.name, self.convert_expression(&var.expr)?);
                }
            }
            Statement::Return(ref term) => {
                match replace(&mut self.return_expression, Some(term.clone())) {
                    Option::None => {}
                    Option::Some(old_term) => {
                        return Err(format!(
                            "multiple return statements\nold:\n{}\nnew:\n{}\n",
                            old_term, term
                        ))
                    }
                };
                let return_expr = self.convert_expression(&term.expr)?;
                replace(&mut self.populated_return_expresion, Some(return_expr));
            }
        };
        Ok(())
    }

    fn convert_expression(&self, expr: &Expression<'a>) -> Result<BlockExpression<'a>, String> {
        match expr {
            Expression::Literal(ref lit) => Ok(BlockExpression::ConstantValue(
                lit.lit.clone(),
                lit.lit.get_type()?,
            )),
            Expression::Variable(ref var) => match self.get_constant_type(var.name) {
                Option::Some(kind) => Ok(BlockExpression::ExternalConstant(var.name, kind)),
                Option::None => match self.get_var_type(var.name) {
                    Option::Some(kind) => Ok(BlockExpression::Var(var.name, kind)),
                    Option::None => Err(format!(
                        "variable '{}' is referenced but not defined",
                        var.name
                    )),
                },
            },
            Expression::Func(ref func) => match self.namespace.get_function(func.name) {
                Option::None => Err(format!(
                    "function invocation: '{}' cannot be made function not found",
                    func.name
                )),
                Option::Some(ref func_data) => {
                    if func_data.args.len() == func.args.len() {
                        let mut arg_vec = Vec::with_capacity(func.args.len());
                        for (index, arg) in func.args.iter().enumerate() {
                            let block_expr = self.convert_expression(arg)?;
                            let expected_type = func_data.args[index].1.clone();
                            let found_type = block_expr.get_type()?;
                            if found_type != expected_type {
                                return Err(format!("expression: '{}' has an error the {} argument to function '{}' is of the incorrect type. Expected type:{} Found type:{}", expr, index, func_data, expected_type, found_type));
                            }
                            arg_vec.push(block_expr);
                        }
                        Ok(BlockExpression::Func(
                            func.name,
                            arg_vec.into_boxed_slice(),
                            func_data.ret,
                        ))
                    } else {
                        Err(format!("function invocation: '{}' has the name of function: '{}' but incorrect argument count. Expected: {} Found: {}", func, func_data, func_data.args.len(), func.args.len()))
                    }
                }
            },
            Expression::Operation(ref op) => {
                let left = Box::new(self.convert_expression(op.left.as_ref())?);
                let right = Box::new(self.convert_expression(op.right.as_ref())?);
                Ok(BlockExpression::Op(left, op.op.clone(), right))
            }
        }
    }

    fn get_constant_type(&self, name: &str) -> Option<TypeData> {
        self.namespace
            .get_constant(name)
            .map(|constant| constant.kind.clone())
    }
    fn get_var_type(&self, name: &str) -> Option<TypeData> {
        self.vars.get(name).map(|var| var.kind.clone())
    }
}


