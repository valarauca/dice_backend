use std::fmt;

use lalrpop_util::ParseError;
use super::value::{TreeParser};

/// AbstractSyntaxTree is the top level of parse. 
///
/// Additional passes are made before a "parse" is
/// complete to ensure that literals are well formed.
pub struct AbstractSyntaxTree<'a> {
    pub ast: Box<[Structures<'a>]>,
}
impl<'a> fmt::Display for AbstractSyntaxTree<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in self.ast.iter() {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}
impl<'a> AbstractSyntaxTree<'a> {
    pub fn new(args: Vec<(Structures<'a>,&'a str)>) -> AbstractSyntaxTree<'a> {
        let ast: Vec<Structures<'a>> = args.into_iter().map(|tup| tup.0).collect();
        let ast = ast.into_boxed_slice();
        AbstractSyntaxTree{ ast }
    }

    /// Parse will attempt to construct an abstract syntax tree from the input
    pub fn parse<'b>(input: &'b str) -> Result<AbstractSyntaxTree<'b>,String> {
        #[inline(always)]
        fn ahead(input: &str, pos: usize) -> Option<usize> {
            input.char_indices().filter(|(i,c)| *c == '\n' && *i <= pos).map(|(i,_)| i).next()
        }
        #[inline(always)]
        fn behind(input: &str, pos: usize) -> Option<usize> {
            input.char_indices().filter(|(i,c)| *c == '\n' && *i >= pos).map(|(i,_)| i).next()
        }
        #[inline(always)]
        fn snippet<'c>(input: &'c str, pos_start: usize, pos_end: usize) -> &'c str {
            let start = ahead(input, pos_start).into_iter().flat_map(|pos| ahead(input, pos)).next().unwrap_or(0usize);
            let end = behind(input, pos_end).into_iter().flat_map(|pos| behind(input, pos)).next().unwrap_or(input.len());
            unsafe{ ::std::str::from_utf8_unchecked(&input.as_bytes()[start..end]) }
        }
        match TreeParser::new().parse(input) {
            Ok(tree) => Ok(tree),
            Err(ParseError::InvalidToken{ location }) => {
                Err(format!("Unable to parse:\n{}\n", snippet(input, location, location)))
            },
            Err(ParseError::UnrecognizedEOF{ location: _, expected: _}) => {
                Err(format!("File terminated before it should"))
            },
            Err(ParseError::UnrecognizedToken{token: (a,_,b), expected }) => {
                Err(format!("Unable to parse:\n{}\n", snippet(input, a, b)))
            },
            Err(_) => {
                unreachable!()
            }
        }
    }
}

/// Literal values.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Literal<'a> {
    Number(i64),
    Boolean(bool),
    EnvirBool(&'a str),
    EnvirNumber(&'a str),
}
impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(ref num) => write!(f, "{}", *num),
            Literal::Boolean(ref var) => if *var {
                write!(f, "true")
            } else {
                write!(f, "false")
            },
            Literal::EnvirBool(ref name) => write!(f, "%b{{{{{}}}}}", name),
            Literal::EnvirNumber(ref name) => write!(f, "%d{{{{{}}}}}", name),
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum TypeData {
    Bool,
    Int,
    CollectionOfBool,
    CollectionOfInt,
}
impl fmt::Display for TypeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeData::Bool => write!(f, "bool"), 
            TypeData::Int => write!(f, "int"),
            TypeData::CollectionOfBool => write!(f, "vec<bool>"),
            TypeData::CollectionOfInt => write!(f, "vec<int>"),
        }
    }
}

/// Operations are things we do to numbers
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Or,
    And,
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"), 
            Operation::Sub => write!(f, "-"), 
            Operation::Mul => write!(f, "*"), 
            Operation::Div => write!(f, "/"), 
            Operation::Or => write!(f, "|"), 
            Operation::And => write!(f, "&"), 
        }
    }
}

/// Statements are a collection of operations
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Statements<'a> {
    data: Box<[Statement<'a>]>,
}
impl<'a> fmt::Display for Statements<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in self.data.iter() {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}
impl<'a> Statements<'a> {
    #[inline(always)]
    pub fn new(arg: Vec<(Statement<'a>,&'a str)>) -> Statements<'a> {
        #[inline(always)] fn mapper<'a>(tup: (Statement<'a>,&'a str)) -> Statement<'a> { tup.0 }
        let collect: Vec<Statement<'a>> = arg.into_iter().map(mapper).collect();
        Statements {
            data: collect.into_boxed_slice(),
        }
    }
}

/// Statements are expressions within a function
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Statement<'a> {
    Variable(VariableDeclaration<'a>),
    Return(TerminalExpression<'a>),
}
impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Variable(ref var) => write!(f,"    let {}: {} = {};\n", var.name, var.kind, var.expr),
            Statement::Return(ref ret) => write!(f,"    return {};\n", ret.expr),
        }
    }
}
impl<'a> Statement<'a> {

    #[inline(always)]
    pub fn new_var(name: &'a str, kind: TypeData, expr: Expression<'a>) -> Statement<'a> {
        Statement::Variable(VariableDeclaration{
            name, kind, expr,
        })
    }

    #[inline(always)]
    pub fn new_ret(expr: Expression<'a>) -> Statement<'a> {
        Statement::Return(TerminalExpression{
            expr,
        })
    }
}

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct TerminalExpression<'a> {
    pub expr: Expression<'a>
}

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct VariableDeclaration<'a> {
    pub name: &'a str,
    pub kind: TypeData,
    pub expr: Expression<'a>
}

/// Structures are top level arguments they exist
/// outside of functions.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Structures<'a> {
    Constant(ConstantDeclaration<'a>),
    Func(FunctionDeclaration<'a>),
    Analyze(AnalysisDeclaration<'a>),
}
impl<'a> fmt::Display for Structures<'a> {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           Structures::Analyze(ref ana) => write!(f, "analyze {};\n", ana.expr),
           Structures::Constant(ref con) => write!(f,"const {}: {} = {};\n", con.name, con.kind, con.expr),
           Structures::Func(ref func) => {
               write!(f, "fn {}( ", func.name)?;
               let last_arg_index = func.args.len()-1;
               for (arg_index, arg) in func.args.iter().enumerate() {
                   if arg_index == last_arg_index {
                       write!(f,"{}: {}", arg.0, arg.1)?;
                   } else {
                       write!(f,"{}: {}, ", arg.0, arg.1)?;
                   }
               }
               write!(f," ) -> {} {{\n {} }}\n", func.ret, func.body)
            }
       }
   }
}
impl<'a> Structures<'a> {

    #[inline(always)]
    pub fn new_const(name: &'a str, kind:TypeData, expr: Expression<'a>) -> Structures<'a> {
        Structures::Constant(ConstantDeclaration {
            name, kind, expr,
        })
    }

    #[inline(always)]
    pub fn new_analysis(expr: Expression<'a>) -> Structures<'a> {
        Structures::Analyze(AnalysisDeclaration {
            expr,
        })
    }

    #[inline(always)]
    pub fn new_func(name: &'a str, args: Vec<(&'a str,&'a str,TypeData,&'a str)>, last_arg: Option<(&'a str, &'a str, TypeData)>, ret: TypeData, body: Statements<'a>) -> Structures<'a> {

        #[inline(always)]
        fn args_mapper<'a>(tup: (&'a str, &'a str, TypeData, &'a str)) -> (&'a str,TypeData) {
            (tup.0,tup.2)
        }
        #[inline(always)]
        fn last_arg_mapper<'a>(tup: (&'a str, &'a str, TypeData)) -> (&'a str,TypeData) {
            (tup.0,tup.2)
        }

        let args: Vec<(&'a str,TypeData)> = args.into_iter()
            .map(args_mapper)
            .chain(last_arg.into_iter().map(last_arg_mapper))
            .collect();
        let args = args.into_boxed_slice();
        Structures::Func(FunctionDeclaration{
            name, args, ret, body,
        })
    }
}

/// ConstantDeclaration is when a constant is declared globally.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct FunctionDeclaration<'a> {
    pub name: &'a str,
    pub args: Box<[(&'a str,TypeData)]>,
    pub ret: TypeData,
    pub body: Statements<'a>
}

/// AnalysisDeclaraction is one of the last top level structures.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct AnalysisDeclaration<'a> {
    pub expr: Expression<'a>
}

/// ConstantDeclaration is when a constant is declared globally.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct ConstantDeclaration<'a> {
   pub name: &'a str,
   pub kind: TypeData,
   pub expr: Expression<'a>,
}

/// Expressions are expressions, things which can be tested.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Expression<'a> {
    Func(FunctionInvocation<'a>),
    Literal(LiteralValue<'a>),
    Operation(OperationResult<'a>),
}
impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Func(ref func) => {
                write!(f,"{}( ", func.name)?;
                let last_arg = func.args.len()-1;
                for (pos,arg) in func.args.iter().enumerate() {
                    if last_arg == pos {
                        write!(f, "{}", arg)?;
                    } else {
                        write!(f,"{}, ", arg)?;
                    }
                }
                write!(f, " )")
            },
            Expression::Literal(ref lit) => {
                write!(f, "{}", lit.lit)
            },
            Expression::Operation(ref op) => {
                write!(f, "( {} {} {} )", op.left, op.op, op.right)
            },
        }
    }
}
impl<'a> Expression<'a> {
    #[inline(always)]
    pub fn new_literal(lit: Literal<'a>) -> Self {
        Expression::Literal(LiteralValue{ lit })
    }

    #[inline(always)]
    pub fn new_function(name: &'a str, args: Vec<(Expression<'a>, &'a str)>, arg: Option<Expression<'a>>) -> Self {
        #[inline(always)] fn tuple_mapper<'a>(arg: (Expression<'a>,&'a str)) -> Expression<'a> { arg.0 }
        Expression::Func(FunctionInvocation{
            name,
            args: args.into_iter().map(tuple_mapper).chain(arg).collect(),
        })
    }

    #[inline(always)]
    pub fn new_operation(left: Expression<'a>, op: Operation, right: Expression<'a>) -> Expression<'a> {
        Expression::Operation(OperationResult{
            left: Box::new(left),
            right: Box::new(right),
            op,
        })
    }
}


#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct FunctionInvocation<'a> {
    pub name: &'a str,
    pub args: Box<[Expression<'a>]>,
}

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct LiteralValue<'a> {
    pub lit: Literal<'a>
}

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct OperationResult<'a> {
    pub left: Box<Expression<'a>>,
    pub op: Operation,
    pub right: Box<Expression<'a>>,
}

