use std::fmt;

use lalrpop_util::ParseError;
use super::value::{TreeParser};
use super::syntaxhelper::{CharacterLookup};

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
    pub fn new(args: Vec<(Structures<'a>)>) -> AbstractSyntaxTree<'a> {
        let ast = args.into_boxed_slice();
        AbstractSyntaxTree{ ast }
    }

    /// Parse will attempt to construct an abstract syntax tree from the input
    pub fn parse<'b>(input: &'b str) -> Result<AbstractSyntaxTree<'b>,String> {
        let index = CharacterLookup::new(input);
        match TreeParser::new().parse(input) {
            Ok(tree) => Ok(tree),
            Err(ParseError::InvalidToken{ location }) => {
                Err(format!("Unable to parse: InvalidToken.\n character: {} line: {} \n {} \n", index.get_char(location), index.get_line_number(location), index.get_line(location)))
            },
            Err(ParseError::UnrecognizedEOF{ location: _, expected: _}) => {
                Err(format!("File terminated before it should"))
            },
            Err(ParseError::UnrecognizedToken{token: (a,_,b), expected }) => {
                Err(format!("Unable to parse: UnreconginzedToken.\n start_line: {} ending_line: {}\n Offending section:\"{}\"\n{}", index.get_line_number(a), index.get_line_number(b), index.get_span(a,b), index.get_span_lines(a,b)))
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
#[test]
fn test_literal_parsing() {
    use super::value::LitParser;

    let parser = LitParser::new();
    assert!( parser.parse("false").unwrap() == Literal::Boolean(false));
    assert!( parser.parse("true").unwrap() == Literal::Boolean(true));
    assert!( parser.parse("15").unwrap() == Literal::Number(15i64));
    assert!( parser.parse("-30").unwrap() == Literal::Number(-30i64));
    assert!( parser.parse("%d{{ENV_VAR}}").unwrap() == Literal::EnvirNumber("ENV_VAR"));
    assert!( parser.parse("%b{{ENV_VAR}}").unwrap() == Literal::EnvirBool("ENV_VAR"));
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

#[test]
fn test_type_data_parsing() {
    use super::value::KindParser;

    let parser = KindParser::new();
    assert!( parser.parse("bool").unwrap() == TypeData::Bool);
    assert!( parser.parse("int").unwrap() == TypeData::Int);
    assert!( parser.parse("vec<bool>").unwrap() == TypeData::CollectionOfBool);
    assert!( parser.parse("vec<int>").unwrap() == TypeData::CollectionOfInt);
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

#[test]
fn test_operation_parsing() {
    use super::value::OpParser;

    let parser = OpParser::new();
    assert!( parser.parse("+").unwrap() == Operation::Add);
    assert!( parser.parse("-").unwrap() == Operation::Sub);
    assert!( parser.parse("*").unwrap() == Operation::Mul);
    assert!( parser.parse("/").unwrap() == Operation::Div);
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

#[test]
fn test_statement_parse() {
    use super::value::StmtParser;
    let parser = StmtParser::new();

    // let arg: int = 15
    let stmt = Statement::Variable(VariableDeclaration{
        name: "arg",
        kind: TypeData::Int,
        expr: Expression::Literal(LiteralValue{
            lit: Literal::Number(15i64),
        })
    });
    assert!(parser.parse("let arg: int = 15").unwrap() == stmt);

    // return false
    let stmt = Statement::Return(TerminalExpression{
            expr: Expression::Literal(LiteralValue{
                lit: Literal::Boolean(false),
            })
    });
    assert!(parser.parse("return false").unwrap() == stmt);
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
    Variable(VariableReference<'a>),
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
            Expression::Variable(ref arg) => {
                write!(f,"{}", arg.name)
            }
        }
    }
}
#[test]
fn test_expression_parsing() {
    use super::value::ExprParser;

    let parser = ExprParser::new();

    // Literal Tests
    assert!(parser.parse("15").unwrap() == Expression::Literal(LiteralValue{ lit: Literal::Number(15i64)}));
    assert!(parser.parse("-35").unwrap() == Expression::Literal(LiteralValue{ lit: Literal::Number(-35i64)}));
    assert!(parser.parse("-35").unwrap() == Expression::Literal(LiteralValue{ lit: Literal::Number(-35i64)}));
    assert!( parser.parse("false").unwrap() == Expression::Literal(LiteralValue{ lit: Literal::Boolean(false) }));
    assert!( parser.parse("true").unwrap() == Expression::Literal(LiteralValue{ lit: Literal::Boolean(true) }));
    assert!( parser.parse("15").unwrap() == Expression::Literal(LiteralValue{ lit: Literal::Number(15i64) }));
    assert!( parser.parse("-30").unwrap() == Expression::Literal(LiteralValue{ lit: Literal::Number(-30i64) }));
    assert!( parser.parse("%d{{ENV_VAR}}").unwrap() == Expression::Literal(LiteralValue{ lit: Literal::EnvirNumber("ENV_VAR") }));
    assert!( parser.parse("%b{{ENV_VAR}}").unwrap() == Expression::Literal(LiteralValue{ lit: Literal::EnvirBool("ENV_VAR") }));

    // Operation Tests
    assert!(parser.parse("( false | true )").unwrap() == Expression::Operation(OperationResult{ left: Box::new(Expression::Literal(LiteralValue{ lit: Literal::Boolean(false)})), op: Operation::Or, right: Box::new(Expression::Literal(LiteralValue{ lit:Literal::Boolean(true)})) }));
    assert!(parser.parse("( 2067 + %d{{INPUT_VALUE_TEST}} )").unwrap() == Expression::Operation(OperationResult{ left: Box::new(Expression::Literal(LiteralValue{ lit: Literal::Number(2067)})), op: Operation::Add, right: Box::new(Expression::Literal(LiteralValue{ lit:Literal::EnvirNumber("INPUT_VALUE_TEST")})) }));

    // Variable Tests
    assert!(parser.parse("helloWorld").unwrap() == Expression::Variable(VariableReference{ name: "helloWorld" }));

    // function test
    assert!(parser.parse("roll_d6(%d{{INPUT_VALUE}})").unwrap() == Expression::Func(FunctionInvocation{ name: "roll_d6", args: vec![Expression::Literal(LiteralValue{ lit: Literal::EnvirNumber("INPUT_VALUE") })].into_boxed_slice() }));
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
    pub fn new_var(name: &'a str) -> Self {
        Expression::Variable(VariableReference{ name })
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
pub struct VariableReference<'a> {
    pub name: &'a str,
}

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct OperationResult<'a> {
    pub left: Box<Expression<'a>>,
    pub op: Operation,
    pub right: Box<Expression<'a>>,
}

