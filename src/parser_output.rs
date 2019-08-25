/// Literal values.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Literal<'a> {
    Number(i64),
    Boolean(bool),
    Str(&'a str),
}

#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum TypeData {
    Bool,
    Int,
    CollectionOfBool,
    CollectionOfInt,
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

/// Statements are a collection of operations
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Statements<'a> {
    data: Box<[Statement<'a>]>,
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
impl<'a> Statement<'a> {

    #[inline(always)]
    pub fn new_var(name: Literal<'a>, kind: TypeData, expr: Expression<'a>) -> Statement<'a> {
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
    pub name: Literal<'a>,
    pub kind: TypeData,
    pub expr: Expression<'a>
}

/// Structures are top level arguments they exist
/// outside of functions.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Structures<'a> {
    Constant(ConstantDeclaration<'a>),
    Func(FunctionDeclaration<'a>),
}
impl<'a> Structures<'a> {

    #[inline(always)]
    pub fn new_const(name: Literal<'a>, kind:TypeData, expr: Expression<'a>) -> Structures<'a> {
        Structures::Constant(ConstantDeclaration {
            name, kind, expr,
        })
    }

    #[inline(always)]
    pub fn new_func(name: Literal<'a>, args: Vec<(Literal<'a>,&'a str,TypeData,&'a str)>, last_arg: Option<(Literal<'a>, &'a str, TypeData)>, ret: TypeData, body: Statements<'a>) -> Structures<'a> {

        #[inline(always)]
        fn args_mapper<'a>(tup: (Literal<'a>, &'a str, TypeData, &'a str)) -> (Literal<'a>,TypeData) {
            (tup.0,tup.2)
        }
        #[inline(always)]
        fn last_arg_mapper<'a>(tup: (Literal<'a>, &'a str, TypeData)) -> (Literal<'a>,TypeData) {
            (tup.0,tup.2)
        }

        let args: Vec<(Literal<'a>,TypeData)> = args.into_iter()
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
    pub name: Literal<'a>,
    pub args: Box<[(Literal<'a>,TypeData)]>,
    pub ret: TypeData,
    pub body: Statements<'a>
}

/// ConstantDeclaration is when a constant is declared globally.
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct ConstantDeclaration<'a> {
   pub name: Literal<'a>,
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
impl<'a> Expression<'a> {
    #[inline(always)]
    pub fn new_literal(lit: Literal<'a>) -> Self {
        Expression::Literal(LiteralValue{ lit })
    }

    #[inline(always)]
    pub fn new_function(name: Literal<'a>, args: Vec<(Expression<'a>, &'a str)>, arg: Option<Expression<'a>>) -> Self {
        #[inline(always)] fn tuple_mapper<'a>(arg: (Expression<'a>,&'a str)) -> Expression<'a> { arg.0 }
        Expression::Func(FunctionInvocation{
            name: name,
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
    pub name: Literal<'a>,
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

