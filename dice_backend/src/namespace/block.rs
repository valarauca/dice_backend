use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::mem::replace;

use super::super::parser_output::{
    Expression, FunctionDeclaration, GetType, Statement, Statements, TerminalExpression, TypeData,
    VariableDeclaration,
};
use super::super::seahasher::DefaultSeaHasher;

use super::blockexpression::BlockExpression;
use super::namespace::Namespace;

/// BasicBlock is in essence a function's body.
/// It can also be used for control structures
/// but presently the language has no control
/// structures.
///
/// This is an intermediate step to producing
/// a "real" SSA.
pub struct BasicBlock<'a> {
    vars: HashMap<&'a str, VariableDeclaration<'a>, DefaultSeaHasher>,
    populated_vars: HashMap<&'a str, BlockExpression<'a>, DefaultSeaHasher>,
    return_expression: Option<TerminalExpression<'a>>,
    populated_return_expresion: Option<BlockExpression<'a>>,
}
impl<'a> BasicBlock<'a> {
    pub fn new(
        names: &Namespace<'a>,
        func: &FunctionDeclaration<'a>,
    ) -> Result<BasicBlock<'a>, String> {
        let mut bb = BasicBlock {
            vars: HashMap::default(),
            populated_vars: HashMap::default(),
            return_expression: None,
            populated_return_expresion: None,
        };
        for (index, (name, kind)) in func.args.iter().enumerate() {
            bb.add_function_vars(func, names, index, name, *kind)?;
        }
        for stmt in func.body.data.iter() {
            bb.add_statement(func, names, stmt)?;
        }
        if bb.return_expression.is_none() {
            // TODO bad error
            return Err(format!("function has no return statement"));
        }
        Ok(bb)
    }

    fn add_statement(
        &mut self,
        func: &FunctionDeclaration<'a>,
        names: &Namespace<'a>,
        stmt: &Statement<'a>,
    ) -> Result<(), String> {
        match stmt {
            Statement::Variable(ref var) => {
                self.insert_variable(func, names, var)?;
            }
            Statement::Return(ref term) => {
                self.insert_return(func, names, term)?;
            }
        };
        Ok(())
    }

    fn convert_expression(
        &self,
        n: &Namespace<'a>,
        expr: &Expression<'a>,
    ) -> Result<BlockExpression<'a>, String> {
        match expr {
            Expression::Literal(ref lit) => Ok(BlockExpression::ConstantValue(
                lit.lit.clone(),
                lit.lit.get_type()?,
            )),
            Expression::Variable(ref var) => match self.get_var_type(n, var.name) {
                Option::Some(kind) => Ok(BlockExpression::Var(var.name, kind)),
                Option::None => Err(format!(
                    "variable '{}' is referenced but not defined",
                    var.name
                )),
            },
            Expression::Func(ref func) => match n.get_function(func.name) {
                Option::None => Err(format!(
                    "function invocation: '{}' cannot be made function not found",
                    func.name
                )),
                Option::Some(ref func_data) => {
                    if func_data.args.len() == func.args.len() {
                        let mut arg_vec = Vec::with_capacity(func.args.len());
                        for (index, arg) in func.args.iter().enumerate() {
                            let block_expr = self.convert_expression(n, arg)?;
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
                let left = Box::new(self.convert_expression(n, op.left.as_ref())?);
                let right = Box::new(self.convert_expression(n, op.right.as_ref())?);
                Ok(BlockExpression::Op(left, op.op.clone(), right))
            }
        }
    }

    /// inserts a function's variable into the local block.
    fn add_function_vars(
        &mut self,
        f: &FunctionDeclaration<'a>,
        n: &Namespace<'a>,
        index: usize,
        name: &'a str,
        kind: TypeData,
    ) -> Result<(), String> {
        if self.is_name_defined(n, name) {
            return Err(format!("within function declaration: '{}' argument: '{}: {}' its name collides with an external variable", f, name, kind));
        }
        self.populated_vars
            .insert(name, BlockExpression::FunctionArg(name, kind));
        Ok(())
    }

    /// inserts a variable into the local block
    fn insert_variable(
        &mut self,
        f: &FunctionDeclaration<'a>,
        n: &Namespace<'a>,
        var: &VariableDeclaration<'a>,
    ) -> Result<(), String> {
        if self.is_name_defined(n, var.name) {
            return Err(format!(
                "variable: '{}' collides with an existing name",
                var
            ));
        }

        // do a typecheck
        let expr = self.convert_expression(n, &var.expr)?;
        let expr_type = expr.get_type()?;
        if expr_type != var.kind {
            return Err(format!("in variable defination: '{}' the expression: '{}' returns type: '{}' while the variable is declared '{}'", var, var.expr, expr_type, var.kind));
        }

        // populate the hash tables
        self.vars.insert(var.name, var.clone());
        self.populated_vars.insert(var.name, expr);
        Ok(())
    }

    fn insert_return(
        &mut self,
        f: &FunctionDeclaration<'a>,
        n: &Namespace<'a>,
        term: &TerminalExpression<'a>,
    ) -> Result<(), String> {
        // guard to avoid multiple returns
        match replace(&mut self.return_expression, Some(term.clone())) {
            Option::None => {}
            Option::Some(old_term) => {
                return Err(format!(
                    "multiple return statements\nold:\n{}\nnew:\n{}\n",
                    old_term, term
                ))
            }
        };

        // type check against function declaration
        let return_expr = self.convert_expression(n, &term.expr)?;
        let return_type = return_expr.get_type()?;
        if return_type != f.ret {
            return Err(format!("return expression: 'return {};' has type of '{}' while the function we are returning: '{}' expects: '{}'", term.expr, return_type, f, f.ret));
        }

        // update one more field
        replace(&mut self.populated_return_expresion, Some(return_expr));
        Ok(())
    }

    fn is_name_defined(&self, n: &Namespace<'a>, name: &str) -> bool {
        n.is_name_defined(name)
            || self.vars.get(name).is_some()
            || self.populated_vars.get(name).is_some()
    }

    fn get_var_type(&self, namespace: &Namespace<'a>, name: &str) -> Option<TypeData> {
        namespace
            .get_constant_type(name)
            .into_iter()
            .chain(self.vars.get(name).map(|var| var.kind)) // variable pool
            .chain(
                self.populated_vars
                    .get(name)
                    .into_iter()
                    .filter_map(|var| var.get_type().ok()),
            ) // possible function args
            .next()
    }
}
