use std::collections::hash_map::Iter;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::mem::replace;

use super::super::parser_output::{
    AbstractSyntaxTree, AnalysisDeclaration, ConstantDeclaration, FunctionDeclaration, Statements,
    Structures, TypeData,
};
use super::super::seahasher::DefaultSeaHasher;

use super::block::BasicBlock;

/// Namespace represents the pre-parsing of the of the AST.
/// It will attempt to ensure there are no collisions with
/// the standard library, or the input program.
#[derive(Default)]
pub struct Namespace<'a> {
    constants: HashMap<&'a str, ConstantDeclaration<'a>, DefaultSeaHasher>,
    functions: HashMap<&'a str, FunctionDeclaration<'a>, DefaultSeaHasher>,
    function_body: HashMap<&'a str, BasicBlock<'a>, DefaultSeaHasher>,
    owndata: Option<BasicBlock<'a>>,
    analysis: Option<AnalysisDeclaration<'a>>,
}
impl<'a> Namespace<'a> {
    /// new constructs an instance of namespace.
    pub fn new(ast: &AbstractSyntaxTree<'a>) -> Result<Namespace<'a>, String> {
        let mut analysis = Namespace::default();
        analysis.populate_std();
        for item in ast.ast.iter() {
            // actions do nothing unless items is of
            // correct enum variance. when not it,
            // returns okay.
            analysis.add_const(item)?;
            analysis.add_function(item)?;
            analysis.add_analysis(item)?;
        }
        for (name, func) in analysis.functions.iter() {
            let bb = BasicBlock::from_func(&analysis, func)?;
            analysis.function_body.insert(name, bb);
        }
        let rootblock = Some(BasicBlock::from_root(&analysis)?);
        analysis.owndata = rootblock;
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

    /// return an iterator over all constants
    pub fn get_all_constants<'b>(&'b self) -> Iter<'b, &'a str, ConstantDeclaration<'a>> {
        self.constants.iter()
    }

    pub fn get_all_function_blocks<'b>(&'b self) -> Iter<'b, &'a str, BasicBlock<'a>> {
        self.function_body.iter()
    }

    /// returns the analysis for this program
    pub fn get_analysis<'b>(&'b self) -> &'b Option<AnalysisDeclaration<'a>> {
        &self.analysis
    }

    /// returns the type for a constatn
    pub fn get_constant_type(&self, name: &str) -> Option<TypeData> {
        self.constants.get(name).map(|constant| constant.kind)
    }

    /// checks if this name already exists
    pub fn is_name_defined(&self, arg: &str) -> bool {
        self.get_constant(arg).is_some() || self.get_function(arg).is_some()
    }

    pub fn get_own_block<'b>(&'b self) -> &'b Option<BasicBlock<'a>> {
        &self.owndata
    }

    fn populate_std(&mut self) {
        self.functions.insert(
            "roll_d6",
            FunctionDeclaration {
                stdlib: true,
                name: "roll_d6",
                args: vec![("number_of_d6_to_roll", TypeData::Int)].into_boxed_slice(),
                ret: TypeData::CollectionOfInt,
                body: Statements {
                    data: vec![].into_boxed_slice(),
                },
            },
        );
        self.functions.insert(
            "roll_d3",
            FunctionDeclaration {
                stdlib: true,
                name: "roll_d3",
                args: vec![("number_of_d3_to_roll", TypeData::Int)].into_boxed_slice(),
                ret: TypeData::CollectionOfInt,
                body: Statements {
                    data: vec![].into_boxed_slice(),
                },
            },
        );
        self.functions.insert(
            "reroll",
            FunctionDeclaration {
                stdlib: true,
                name: "reroll",
                args: vec![
                    ("max_value_roll_can_return_inclusive", TypeData::Int),
                    ("min_value_roll_can_return_inclusive", TypeData::Int),
                    ("filters_to_reroll", TypeData::CollectionOfBool),
                    ("collection_to_be_re_rolled", TypeData::CollectionOfInt),
                ]
                .into_boxed_slice(),
                ret: TypeData::CollectionOfInt,
                body: Statements {
                    data: vec![].into_boxed_slice(),
                },
            },
        );
        self.functions.insert(
            "roll",
            FunctionDeclaration {
                stdlib: true,
                name: "roll",
                args: vec![
                    ("max_value_roll_can_return_inclusive", TypeData::Int),
                    ("min_value_roll_can_return_inclusive", TypeData::Int),
                    ("number_of_rolls_to_make", TypeData::Int),
                ]
                .into_boxed_slice(),
                ret: TypeData::CollectionOfInt,
                body: Statements {
                    data: vec![].into_boxed_slice(),
                },
            },
        );
        self.functions.insert(
            "filter",
            FunctionDeclaration {
                stdlib: true,
                name: "filter",
                args: vec![
                    (
                        "collection_of_tests_to_filter_with",
                        TypeData::CollectionOfBool,
                    ),
                    ("collection_of_data_to_filter", TypeData::CollectionOfInt),
                ]
                .into_boxed_slice(),
                ret: TypeData::CollectionOfBool,
                body: Statements {
                    data: vec![].into_boxed_slice(),
                },
            },
        );
        self.functions.insert(
            "count",
            FunctionDeclaration {
                stdlib: true,
                name: "count",
                args: vec![("collection_to_sum_over", TypeData::CollectionOfBool)]
                    .into_boxed_slice(),
                ret: TypeData::Int,
                body: Statements {
                    data: vec![].into_boxed_slice(),
                },
            },
        );
        self.functions.insert(
            "sum",
            FunctionDeclaration {
                stdlib: true,
                name: "sum",
                args: vec![("collection_to_sum_with", TypeData::CollectionOfInt)]
                    .into_boxed_slice(),
                ret: TypeData::Int,
                body: Statements {
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
