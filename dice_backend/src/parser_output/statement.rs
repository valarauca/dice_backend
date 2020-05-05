use std::fmt;

use super::expression::Expression;
use super::literal::Literal;
use super::literalvalue::LiteralValue;
use super::terminalexpression::TerminalExpression;
use super::typedata::TypeData;
use super::variabledeclaration::VariableDeclaration;

/// Statements are expressions within a function
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Statement<'a> {
    Variable(VariableDeclaration<'a>),
    Return(TerminalExpression<'a>),
}
impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Variable(ref var) => {
                write!(f, "    let {}: {} = {};\n", var.name, var.kind, var.expr)
            }
            Statement::Return(ref ret) => write!(f, "    return {};\n", ret.expr),
        }
    }
}
impl<'a> Statement<'a> {
    pub fn get_variable_declaration<'b>(s: &'b Self) -> Option<&'b VariableDeclaration> {
        match s {
            Statement::Variable(ref var) => Some(var),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn new_var(name: &'a str, kind: TypeData, expr: Expression<'a>) -> Statement<'a> {
        Statement::Variable(VariableDeclaration { name, kind, expr })
    }

    #[inline(always)]
    pub fn new_ret(expr: Expression<'a>) -> Statement<'a> {
        Statement::Return(TerminalExpression { expr })
    }
}

#[test]
fn test_statement_parse() {
    use super::super::value::StmtParser;
    let parser = StmtParser::new();

    // let arg: int = 15
    let stmt = Statement::Variable(VariableDeclaration {
        name: "arg",
        kind: TypeData::Int,
        expr: Expression::Literal(LiteralValue {
            lit: Literal::Number(15i8),
        }),
    });
    assert!(parser.parse("let arg: int = 15;").unwrap() == stmt);

    // return false
    let stmt = Statement::Return(TerminalExpression {
        expr: Expression::Literal(LiteralValue {
            lit: Literal::Boolean(false),
        }),
    });
    assert!(parser.parse("return false;").unwrap() == stmt);
}
