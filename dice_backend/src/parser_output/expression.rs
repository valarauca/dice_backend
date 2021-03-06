use std::fmt;

use super::functioninvoke::FunctionInvocation;
use super::literal::Literal;
use super::literalvalue::LiteralValue;
use super::operation::Operation;
use super::operationresult::OperationResult;
use super::typedata::TypeData;
use super::varreference::VariableReference;

use super::GetType;

/// Expressions are expressions, things which can be tested.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expression<'a> {
    Func(FunctionInvocation<'a>),
    Literal(LiteralValue<'a>),
    Operation(OperationResult<'a>),
    Variable(VariableReference<'a>),
}
impl<'a> GetType for Expression<'a> {
    fn requires_namespace(&self) -> bool {
        match self {
            Expression::Func(ref func) => func.requires_namespace(),
            Expression::Literal(ref lit) => lit.requires_namespace(),
            Expression::Operation(ref op) => op.requires_namespace(),
            Expression::Variable(ref var) => var.requires_namespace(),
        }
    }

    fn get_type(&self) -> Result<TypeData, String> {
        match self {
            Expression::Func(ref func) => func.get_type(),
            Expression::Literal(ref lit) => lit.get_type(),
            Expression::Operation(ref op) => op.get_type(),
            Expression::Variable(ref var) => var.get_type(),
        }
    }
}
impl<'a> fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Func(ref func) => {
                write!(f, "{}( ", func.name)?;
                let last_arg = func.args.len() - 1;
                for (pos, arg) in func.args.iter().enumerate() {
                    if last_arg == pos {
                        write!(f, "{}", arg)?;
                    } else {
                        write!(f, "{}, ", arg)?;
                    }
                }
                write!(f, " )")
            }
            Expression::Literal(ref lit) => write!(f, "{}", lit.lit),
            Expression::Operation(ref op) => write!(f, "( {} {} {} )", op.left, op.op, op.right),
            Expression::Variable(ref arg) => write!(f, "{}", arg.name),
        }
    }
}
#[test]
fn test_expression_parsing() {
    use super::super::value::ExprParser;

    let parser = ExprParser::new();

    // Literal Tests
    assert!(
        parser.parse("15").unwrap()
            == Expression::Literal(LiteralValue {
                lit: Literal::Number(15i8)
            })
    );
    assert!(
        parser.parse("-35").unwrap()
            == Expression::Literal(LiteralValue {
                lit: Literal::Number(-35i8)
            })
    );
    assert!(
        parser.parse("-35").unwrap()
            == Expression::Literal(LiteralValue {
                lit: Literal::Number(-35i8)
            })
    );
    assert!(
        parser.parse("false").unwrap()
            == Expression::Literal(LiteralValue {
                lit: Literal::Boolean(false)
            })
    );
    assert!(
        parser.parse("true").unwrap()
            == Expression::Literal(LiteralValue {
                lit: Literal::Boolean(true)
            })
    );
    assert!(
        parser.parse("15").unwrap()
            == Expression::Literal(LiteralValue {
                lit: Literal::Number(15i8)
            })
    );
    assert!(
        parser.parse("-30").unwrap()
            == Expression::Literal(LiteralValue {
                lit: Literal::Number(-30i8)
            })
    );
    assert!(
        parser.parse("%d{{ENV_VAR}}").unwrap()
            == Expression::Literal(LiteralValue {
                lit: Literal::EnvirNumber("ENV_VAR")
            })
    );
    assert!(
        parser.parse("%b{{ENV_VAR}}").unwrap()
            == Expression::Literal(LiteralValue {
                lit: Literal::EnvirBool("ENV_VAR")
            })
    );

    // Operation Tests
    assert!(
        parser.parse("( false | true )").unwrap()
            == Expression::Operation(OperationResult {
                left: Box::new(Expression::Literal(LiteralValue {
                    lit: Literal::Boolean(false)
                })),
                op: Operation::Or,
                right: Box::new(Expression::Literal(LiteralValue {
                    lit: Literal::Boolean(true)
                }))
            })
    );
    assert!(
        parser.parse("( 20 + %d{{INPUT_VALUE_TEST}} )").unwrap()
            == Expression::Operation(OperationResult {
                left: Box::new(Expression::Literal(LiteralValue {
                    lit: Literal::Number(20)
                })),
                op: Operation::Add,
                right: Box::new(Expression::Literal(LiteralValue {
                    lit: Literal::EnvirNumber("INPUT_VALUE_TEST")
                }))
            })
    );

    // Variable Tests
    assert!(
        parser.parse("helloWorld").unwrap()
            == Expression::Variable(VariableReference { name: "helloWorld" })
    );

    // function test
    assert!(
        parser.parse("roll_d6(%d{{INPUT_VALUE}})").unwrap()
            == Expression::Func(FunctionInvocation {
                name: "roll_d6",
                args: vec![Expression::Literal(LiteralValue {
                    lit: Literal::EnvirNumber("INPUT_VALUE")
                })]
                .into_boxed_slice()
            })
    );
}
impl<'a> Expression<'a> {
    #[inline(always)]
    pub fn new_literal(lit: Literal<'a>) -> Self {
        Expression::Literal(LiteralValue { lit })
    }

    #[inline(always)]
    pub fn new_function(
        name: &'a str,
        args: Vec<(Expression<'a>, &'a str)>,
        arg: Option<Expression<'a>>,
    ) -> Self {
        #[inline(always)]
        fn tuple_mapper<'a>(arg: (Expression<'a>, &'a str)) -> Expression<'a> {
            arg.0
        }
        Expression::Func(FunctionInvocation {
            name,
            args: args.into_iter().map(tuple_mapper).chain(arg).collect(),
        })
    }

    #[inline(always)]
    pub fn new_var(name: &'a str) -> Self {
        Expression::Variable(VariableReference { name })
    }

    #[inline(always)]
    pub fn new_operation(
        left: Expression<'a>,
        op: Operation,
        right: Expression<'a>,
    ) -> Expression<'a> {
        Expression::Operation(OperationResult {
            left: Box::new(left),
            right: Box::new(right),
            op,
        })
    }
}
