use std::collections::hash_map::Iter;
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
#[derive(Default)]
pub struct BasicBlock<'a> {
    vars: HashMap<&'a str, VariableDeclaration<'a>, DefaultSeaHasher>,
    populated_vars: HashMap<&'a str, BlockExpression<'a>, DefaultSeaHasher>,
    populated_return_expresion: Option<BlockExpression<'a>>,
}
impl<'a> BasicBlock<'a> {
    /// new constructs a basic block from a function's declaration within a namespace
    pub fn from_func(
        names: &Namespace<'a>,
        func: &FunctionDeclaration<'a>,
    ) -> Result<BasicBlock<'a>, String> {
        let mut bb = BasicBlock::default();
        for (index, (name, kind)) in func.args.iter().enumerate() {
            bb.add_function_vars(func, names, index, name, *kind)?;
        }
        for stmt in func.body.data.iter() {
            bb.add_statement(func, names, stmt)?;
        }
        Ok(bb)
    }

    /// handles only converting the root expression not individual functions
    pub fn from_root(names: &Namespace<'a>) -> Result<BasicBlock<'a>, String> {
        let mut bb = BasicBlock {
            vars: HashMap::default(),
            populated_vars: HashMap::default(),
            populated_return_expresion: None,
        };
        for (name, value) in names.get_all_constants() {
            let expr = bb.convert_expression(names, &value.expr)?;
            let expr_type = expr.get_type()?;
            if value.kind != expr_type {
                return Err(format!("constant declaration: '{}' is in error. Expression: '{}' yeilds type: '{}' but we are binding type: '{}'", value, &value.expr, expr_type, value.kind));
            }
            bb.populated_vars.insert(name, expr);
        }

        let analysis = match names.get_analysis() {
            Option::Some(analysis) => analysis,
            Option::None => {
                return Err(format!(
                    "program contains no analysis directive. How do we report?"
                ));
            }
        };
        let expr = bb.convert_expression(names, &analysis.expr)?;
        replace(&mut bb.populated_return_expresion, Some(expr));
        Ok(bb)
    }

    pub fn get_return<'b>(&'b self) -> &'b Option<BlockExpression<'a>> {
        &self.populated_return_expresion
    }

    pub fn get_vars<'b>(&'b self) -> Iter<'b, &'a str, BlockExpression<'a>> {
        self.populated_vars.iter()
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
            Expression::Literal(ref lit) => Ok(BlockExpression::lit(&lit.lit)?),
            Expression::Variable(ref var) => Ok(self.convert_expr_var_name(n, var.name)?),
            Expression::Func(ref func) => {
                // lookup function in namespace
                let func_data = match n.get_function(func.name) {
                    Option::None => {
                        return Err(format!("function invocation: '{}' cannot be resolved. no function of that name is defined", func));
                    }
                    Option::Some(func_data) => func_data,
                };
                // ensure the call provides enough arguments
                let declared_args_count: usize = func_data.args.len();
                let referenced_args_count: usize = func.args.len();
                if declared_args_count != referenced_args_count {
                    return Err(format!("function invocation: '{}' has the name of function: '{}' but incorrect argument count. Expected: {} Found: {}", func, func_data, declared_args_count, referenced_args_count));
                }

                // iterate over the arguments & convert their expressions recursively
                // atttempt to perform type checking recursively as well.
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
                Ok(BlockExpression::func(func.name, arg_vec, func_data.ret)?)
            }
            Expression::Operation(ref op) => {
                let left = self.convert_expression(n, op.left.as_ref())?;
                let right = self.convert_expression(n, op.right.as_ref())?;
                Ok(BlockExpression::op(left, op.op.clone(), right)?)
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
            .insert(name, BlockExpression::FunctionArg(name, index, kind));
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
        self.populated_vars.insert(var.name, expr);
        Ok(())
    }

    fn insert_return(
        &mut self,
        f: &FunctionDeclaration<'a>,
        n: &Namespace<'a>,
        term: &TerminalExpression<'a>,
    ) -> Result<(), String> {
        // type check against function declaration
        let return_expr = self.convert_expression(n, &term.expr)?;
        let return_type = return_expr.get_type()?;
        if return_type != f.ret {
            return Err(format!("return expression: 'return {};' has type of '{}' while the function we are returning: '{}' expects: '{}'", term.expr, return_type, f, f.ret));
        }

        // update return field
        match replace(&mut self.populated_return_expresion, Some(return_expr)) {
            Option::None => {}
            Option::Some(old_term) => {
                return Err(format!(
                    "multiple return statements\nold:\n{}\nnew:\n{}\n",
                    old_term, term
                ))
            }
        };
        Ok(())
    }

    fn is_name_defined(&self, n: &Namespace<'a>, name: &str) -> bool {
        n.is_name_defined(name) || self.populated_vars.get(name).is_some()
    }

    // convert_expr_var_name handles the messiness of determining _what kind of variable_
    // is being referenced.
    fn convert_expr_var_name(
        &self,
        namespace: &Namespace<'a>,
        name: &'a str,
    ) -> Result<BlockExpression<'a>, String> {
        match namespace.get_constant(name) {
            Option::None => {}
            Option::Some(ref constant_dec) => {
                return Ok(BlockExpression::ExternalConstant(
                    name,
                    constant_dec.kind.clone(),
                ));
            }
        };
        match self.populated_vars.get(name) {
            Option::Some(&BlockExpression::FunctionArg(_, ref index, ref kind)) => {
                return Ok(BlockExpression::FunctionArg(
                    name,
                    index.clone(),
                    kind.clone(),
                ));
            }
            Option::Some(ref block) => {
                return Ok(BlockExpression::Var(name, block.get_type()?));
            }
            Option::None => {}
        };
        Err(format!("variable name:'{}' is not defined", name))
    }
}
