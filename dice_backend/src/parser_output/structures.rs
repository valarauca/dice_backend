use std::fmt;

use super::analysisdeclaration::AnalysisDeclaration;
use super::constantdeclaration::ConstantDeclaration;
use super::expression::Expression;
use super::functiondeclaration::FunctionDeclaration;
use super::typedata::TypeData;

use super::Statements;

/// Structures are top level arguments they exist
/// outside of functions.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Structures<'a> {
    Constant(ConstantDeclaration<'a>),
    Func(FunctionDeclaration<'a>),
    Analyze(AnalysisDeclaration<'a>),
}
impl<'a> fmt::Display for Structures<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Structures::Analyze(ref ana) => write!(f, "analyze {};\n", ana.expr),
            Structures::Constant(ref con) => {
                write!(f, "const {}: {} = {};\n", con.name, con.kind, con.expr)
            }
            Structures::Func(ref func) => {
                write!(f, "fn {}( ", func.name)?;
                let last_arg_index = func.args.len() - 1;
                for (arg_index, arg) in func.args.iter().enumerate() {
                    if arg_index == last_arg_index {
                        write!(f, "{}: {}", arg.0, arg.1)?;
                    } else {
                        write!(f, "{}: {}, ", arg.0, arg.1)?;
                    }
                }
                write!(f, " ) -> {} {{\n {} }}\n", func.ret, func.body)
            }
        }
    }
}
impl<'a> Structures<'a> {
    pub fn to_func<'b>(s: &'b Structures<'a>) -> Option<&'b FunctionDeclaration<'a>> {
        match s {
            Structures::Func(func) => Some(func),
            _ => None,
        }
    }

    pub fn to_const<'b>(s: &'b Structures<'a>) -> Option<&'b ConstantDeclaration<'a>> {
        match s {
            Structures::Constant(cons) => Some(cons),
            _ => None,
        }
    }

    pub fn to_analysis<'b>(s: &'b Structures<'a>) -> Option<&'b AnalysisDeclaration<'a>> {
        match s {
            Structures::Analyze(cons) => Some(cons),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn new_const(name: &'a str, kind: TypeData, expr: Expression<'a>) -> Structures<'a> {
        Structures::Constant(ConstantDeclaration { name, kind, expr })
    }

    #[inline(always)]
    pub fn new_analysis(expr: Expression<'a>) -> Structures<'a> {
        Structures::Analyze(AnalysisDeclaration { expr })
    }

    #[inline(always)]
    pub fn new_func(
        name: &'a str,
        args: Vec<(&'a str, &'a str, TypeData, &'a str)>,
        last_arg: Option<(&'a str, &'a str, TypeData)>,
        ret: TypeData,
        body: Statements<'a>,
    ) -> Structures<'a> {
        #[inline(always)]
        fn args_mapper<'a>(tup: (&'a str, &'a str, TypeData, &'a str)) -> (&'a str, TypeData) {
            (tup.0, tup.2)
        }
        #[inline(always)]
        fn last_arg_mapper<'a>(tup: (&'a str, &'a str, TypeData)) -> (&'a str, TypeData) {
            (tup.0, tup.2)
        }

        let args: Vec<(&'a str, TypeData)> = args
            .into_iter()
            .map(args_mapper)
            .chain(last_arg.into_iter().map(last_arg_mapper))
            .collect();
        let args = args.into_boxed_slice();
        Structures::Func(FunctionDeclaration {
            name,
            args,
            ret,
            body,
        })
    }
}
