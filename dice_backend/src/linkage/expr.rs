use super::super::cfgbuilder::{ExpressionCollection, HashedExpression, Identifier};
use super::super::parser_output::{Literal, Operation, TypeData};

/// LinkageExpression is the results of identifiers being removed.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LinkageExpression<'a> {
    Constant(Literal<'a>, TypeData),
    Operation(
        Box<LinkagedExpression<'a>>,
        Operation,
        Box<LinkagedExpression<'a>>,
        TypeData,
    ),
    Function(Identifier, Box<[LinkageExpression<'a>]>, TypeData),
}
impl<'a> LinkageExpression<'a> {
    pub fn get_type(&self) -> TypeData {
        match self {
            &LinkageExpression::Constant(_, ref kind) => kind.clone(),
            &LinkageExpression::Operation(_, _, _, ref kind) => kind.clone(),
            &LinkageExpression::Function(_, _, ref kind) => kind.clone(),
        }
    }

    pub fn expr(
        expression: &HashedExpression<'a>,
        n: &ExpressionCollection<'a>,
    ) -> LinkageExpression<'a> {
        match expression {
            &HashedExpression::FunctionArg(_, _, _) => unreachable!(),
            &HashedExpression::ConstantValue(ref lit, ref kind) => {
                LinkageExpression::Constant(lit.clone(), kind.clone())
            }
            &HashedExpression::Var(ref id, ref kind)
            | &HashedExpression::ExternalConstant(ref id, ref kind) => {
                let new_expr = LinkageExpression::expr(n.get_variable(id), n);
                assert!(new_expr.get_type() == kind.clone());
                new_expr
            }
            &HashedExpression::Func(ref id, ref args, ref kind) => {
                if n.is_function_stdlib(id) {
                    // no magic here
                    let new_args = args
                        .iter()
                        .map(|arg| LinkageExpression::expr(n.get_variable(id)))
                        .collect::<Vec<_>>()
                        .into_boxed_slice();
                    LinkageExpression::Func(id.clone(), new_args, kind.clone())
                } else {
                    unreachable!()
                }
            }
            &HashedExpression::Op(ref left, ref op, ref right, ref kind) => {}
        }
    }
}
