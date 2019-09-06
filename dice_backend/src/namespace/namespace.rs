use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::mem::replace;

use super::super::parser_output::{
    AnalysisDeclaration, ConstantDeclaration, FunctionDeclaration, Statements, Structures, TypeData,
};
use super::super::seahasher::DefaultSeaHasher;

/// Namespace represents the pre-parsing of the of the AST.
/// It will attempt to ensure there are no collisions with
/// the standard library, or the input program.
pub struct Namespace<'a> {
    constants: HashMap<&'a str, ConstantDeclaration<'a>, DefaultSeaHasher>,
    functions: HashMap<&'a str, FunctionDeclaration<'a>, DefaultSeaHasher>,
    analysis: Option<AnalysisDeclaration<'a>>,
}
impl<'a> Namespace<'a> {
    /// new constructs an instance of namespace.
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

    pub fn get_constant_type(&self, name: &str) -> Option<TypeData> {
        self.constants.get(name).map(|constant| constant.kind)
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
        let lambda = |arg: &'b ConstantDeclaration<'a>| -> Result<(), String> {
            // check functions first b/c no side effects
            match self.functions.get(arg.name) {
                Option::Some(is_a_func) => {
                    return Err(format!(
                        "constant named=\"{}\" cannot be declared, function=\"{}\" uses that name",
                        is_a_func.name, is_a_func.name
                    ))
                }
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
        let lambda = |arg: &'b FunctionDeclaration<'a>| -> Result<(), String> {
            match self.constants.get(arg.name) {
                Option::Some(is_a_const) => {
                    return Err(format!(
                        "function named=\"{}\" cannot be declared, constant=\"{}\" uses that name",
                        is_a_const.name, is_a_const.name
                    ))
                }
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
