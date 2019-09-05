//! credit to: https://pp.ipd.kit.edu/uploads/publikationen/braun13cc.pdf
//!
//! but as an uninformed observer by querying the call chain lazily you are
//! just building a CFG, just an emergent one of causality.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::mem::replace;

use super::parser_output::*;
use super::seahash::SeaHasher;
use super::seahasher::DefaultSeaHasher;

/// Namespace represents the pre-parsing of the of the AST.
/// It will attempt to ensure there are no collisions with
/// the standard library, or the input program.
pub struct Namespace<'a> {
    constants: HashMap<&'a str, ConstantDeclaration<'a>, DefaultSeaHasher>,
    functions: HashMap<&'a str, FunctionDeclaration<'a>, DefaultSeaHasher>,
    analysis: Option<AnalysisDeclaration<'a>>,
}
impl<'a> Namespace<'a> {
    pub fn new<'b>(args: &'b [Structures<'a>]) -> Result<Namespace<'a>, String> {
        let mut analysis = Namespace {
            constants: HashMap::default(),
            functions: HashMap::default(),
            analysis: None,
        };
        analysis.populate_std();
        for item in args {
            // actions do nothing unless items is of
            // correct enum variance. when not it,
            // returns okay.
            analysis.add_const(item)?;
            analysis.add_function(item)?;
            analysis.add_analysis(item)?;
        }
        Ok(analysis)
    }

    /// returns a function declaration for a specific name to allow for argument &
    /// and type checking.
    pub fn get_function<'b>(&'b self, arg: &str) -> Option<&'b FunctionDeclaration<'a>> {
        self.functions.get(arg)
    }

    /// returns a constant declaration for typing checking and validation.
    pub fn get_constant<'b>(&'b self, arg: &str) -> Option<&'b ConstantDeclaration<'a>> {
        self.constants.get(arg)
    }

    /// checks if this name already exists
    pub fn is_name_defined(&self, arg: &str) -> bool {
        self.get_constant(arg).is_some() || self.get_function(arg).is_some()
    }

    fn populate_std(&mut self) {
        self.functions.insert(
            "roll_d6",
            FunctionDeclaration {
                name: "roll_d6",
                args: vec![("num", TypeData::Int)].into_boxed_slice(),
                ret: TypeData::CollectionOfInt,
                body: Statements {
                    stdlib: true,
                    data: vec![].into_boxed_slice(),
                },
            },
        );
        self.functions.insert(
            "roll_d3",
            FunctionDeclaration {
                name: "roll_d3",
                args: vec![("num", TypeData::Int)].into_boxed_slice(),
                ret: TypeData::CollectionOfInt,
                body: Statements {
                    stdlib: true,
                    data: vec![].into_boxed_slice(),
                },
            },
        );
        self.functions.insert(
            "roll",
            FunctionDeclaration {
                name: "roll",
                args: vec![
                    ("max", TypeData::Int),
                    ("min", TypeData::Int),
                    ("num", TypeData::Int),
                ]
                .into_boxed_slice(),
                ret: TypeData::CollectionOfInt,
                body: Statements {
                    stdlib: true,
                    data: vec![].into_boxed_slice(),
                },
            },
        );
        self.functions.insert(
            "filter",
            FunctionDeclaration {
                name: "filter",
                args: vec![
                    ("test", TypeData::CollectionOfBool),
                    ("collection", TypeData::CollectionOfInt),
                ]
                .into_boxed_slice(),
                ret: TypeData::CollectionOfBool,
                body: Statements {
                    stdlib: true,
                    data: vec![].into_boxed_slice(),
                },
            },
        );
        self.functions.insert(
            "sum",
            FunctionDeclaration {
                name: "sum",
                args: vec![("collection", TypeData::CollectionOfInt)].into_boxed_slice(),
                ret: TypeData::Int,
                body: Statements {
                    stdlib: true,
                    data: vec![].into_boxed_slice(),
                },
            },
        );
    }

    fn add_const<'b>(&mut self, arg: &'b Structures<'a>) -> Result<(), String> {
        let lambda =
            |arg: &'b ConstantDeclaration<'a>| -> Result<(), String> {
                // check functions first b/c no side effects
                match self.functions.get(arg.name) {
                    Option::Some(is_a_func) => return Err(format!(
                        "constant named=\"{}\" cannot be declared, function=\"{}\" uses that name",
                        is_a_func.name, is_a_func.name
                    )),
                    Option::None => {}
                };
                match self.constants.insert(arg.name, arg.clone()) {
                    Option::Some(already_exists) => {
                        return Err(format!(
                            "const named=\"{}\" already exists",
                            already_exists.name
                        ))
                    }
                    Option::None => {}
                };
                Ok(())
            };
        Structures::to_const(arg)
            .into_iter()
            .map(lambda)
            .next()
            .unwrap_or(Ok(()))
    }
    fn add_analysis<'b>(&mut self, arg: &'b Structures<'a>) -> Result<(), String> {
        let lambda = |arg: &'b AnalysisDeclaration<'a>| -> Result<(), String> {
            match replace(&mut self.analysis, Some(arg.clone())) {
                Option::Some(old) => Err(format!("analyze statement is already declared\n\n{}\n\nsecond declaration\n\n{}\n\n is error", old, arg)),
                Option::None => Ok(()),
            }
        };
        Structures::to_analysis(arg)
            .into_iter()
            .map(lambda)
            .next()
            .unwrap_or(Ok(()))
    }
    fn add_function<'b>(&mut self, arg: &'b Structures<'a>) -> Result<(), String> {
        let lambda =
            |arg: &'b FunctionDeclaration<'a>| -> Result<(), String> {
                match self.constants.get(arg.name) {
                    Option::Some(is_a_const) => return Err(format!(
                        "function named=\"{}\" cannot be declared, constant=\"{}\" uses that name",
                        is_a_const.name, is_a_const.name
                    )),
                    Option::None => {}
                };
                match self.functions.insert(arg.name, arg.clone()) {
                    Option::Some(already_exists) => {
                        return Err(format!(
                            "function named=\"{}\" already exists",
                            already_exists.name
                        ))
                    }
                    Option::None => {}
                };
                Ok(())
            };
        Structures::to_func(arg)
            .into_iter()
            .map(lambda)
            .next()
            .unwrap_or(Ok(()))
    }
}

/// BlockExpression is the result of expresion lowering.
/// when preformed it. Block Expressions, unlike normal
/// expressions are not a recrusive data type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlockExpression<'a> {
    ConstantValue(Literal<'a>, TypeData),
    ExternalConstant(&'a str, TypeData),
    Func(&'a str, Box<[BlockExpression<'a>]>, TypeData),
    Var(&'a str, TypeData),
    Op(
        Box<BlockExpression<'a>>,
        Operation,
        Box<BlockExpression<'a>>,
    ),
}
impl<'a> BlockExpression<'a> {
    /// resolving the typing data for the block expression
    fn get_type(&self) -> Result<TypeData, String> {
        match self {
            BlockExpression::ConstantValue(_, kind) => Ok(kind.clone()),
            BlockExpression::ExternalConstant(_, kind) => Ok(kind.clone()),
            BlockExpression::Func(_, _, kind) => Ok(kind.clone()),
            BlockExpression::Var(_, kind) => Ok(kind.clone()),
            BlockExpression::Op(ref left, op, ref right) => match op {
                Operation::Sub | Operation::Mul | Operation::Div | Operation::Add => {
                    match (left.get_type()?, right.get_type()?) {
                        (TypeData::Int, TypeData::Int) => Ok(TypeData::Int),
                        (TypeData::Int, TypeData::CollectionOfInt) => Ok(TypeData::CollectionOfInt),
                        (TypeData::CollectionOfInt, TypeData::Int) => Ok(TypeData::Int),
                        (TypeData::CollectionOfInt, TypeData::CollectionOfInt) => {
                            Ok(TypeData::CollectionOfInt)
                        }
                        // TODO this sucks
                        (left, right) => Err(format!(
                            "type error. Expression on ({} {} {}) is illegal",
                            left, op, right
                        )),
                    }
                }
                Operation::Equal
                | Operation::GreaterThan
                | Operation::LessThan
                | Operation::GreaterThanEqual
                | Operation::LessThanEqual => match (left.get_type()?, right.get_type()?) {
                    (TypeData::Int, TypeData::Int) => Ok(TypeData::Bool),
                    (TypeData::Int, TypeData::CollectionOfInt) => Ok(TypeData::CollectionOfBool),
                    (TypeData::CollectionOfInt, TypeData::Int) => Ok(TypeData::CollectionOfBool),
                    (TypeData::CollectionOfInt, TypeData::CollectionOfInt) => {
                        Ok(TypeData::CollectionOfBool)
                    }
                    // TODO this sucks
                    (left, right) => Err(format!(
                        "type error. Expression on ({} {} {}) is illegal",
                        left, op, right
                    )),
                },
                Operation::Or | Operation::And => match (left.get_type()?, right.get_type()?) {
                    (TypeData::Bool, TypeData::Bool) => Ok(TypeData::Bool),
                    (TypeData::Bool, TypeData::CollectionOfBool) => Ok(TypeData::CollectionOfBool),
                    (TypeData::CollectionOfBool, TypeData::Bool) => Ok(TypeData::CollectionOfBool),
                    (TypeData::CollectionOfBool, TypeData::CollectionOfBool) => {
                        Ok(TypeData::CollectionOfBool)
                    }
                    // TODO this sucks
                    (left, right) => Err(format!(
                        "type error. Expression on ({} {} {}) is illegal",
                        left, op, right
                    )),
                },
            },
        }
    }

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
                lit.lit.get_type(),
            )),
            Expression::Variable(ref var) => match self.get_constant_type(var.name) {
                Option::Some(kind) => Ok(BlockExpression::ExternalConstant(var.name, kind)),
                Option::None => match self.get_var_type(var.name) {
                    Option::Some(kind) => {
                       Ok(BlockExpression::Var(var.name, kind))
                    },
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

