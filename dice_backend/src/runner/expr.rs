
use super::super::cfgbuilder::{HashedExpression,Identifier,ExpressionCollection};
use super::super::parser_output::{Operation,Literal};

use super::stack::{CallStack};

/// Inlined Expression contains the very base values
#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum InlinedExpression<'a> {
    StdLibFunc(&'a str, Box<InlinedExpression<'a>>),
    Operation(Box<InlinedExpression<'a>>, Operation, Box<InlinedExpression<'a>>),
    Constant(Literal<'a>),
}
impl<'a> InlinedExpression<'a> {

    /// Build an inlined expression from context
    pub fn new<'b>(expr: &HashedExpression<'a>, stack: &mut CallStack<'a,'b>)-> Option<InlinedExpression<'a>> {
        match expr {
            &HashedExpression::ConstantValue(ref literal, _) => {
                Some(InlinedExpression::Constant(literal.clone()))
            },
            &HashedExpression::ExternalConstant(ref id, ) |
            &HashedExpression::Var(ref id, _) => {
                // resolve the expression that defines the variable
                // convert that recursively
                n.get_variable(id).into_iter().map(|expr| InlinedExpression::new(expr, stack)).next()
            },
            &HashedExpression::FunctionArg(ref name, ref index, _) => {
            },
            &HashedExpression::Func(ref id, ref args, _) => {
                // convert all the arguments into expressions
                let new_args = args.iter().flat_map(|argument| n.get_expr(ctx, argument)).map(|expr| InlinedExpression::new(expr, n, ctx)).collect::<Vec<_>>();
                // internally a lot of these ooperations use Option<T>
                // there should never be an Option::None but better
                //  safe then sorry.
                assert_eq!(args.len(), new_args.len());
                // can we inline this function's body?
                if n.is_function_stdlib(id) {
                    Some(InlinedExpression::Func(id.clone(), new_args.into_boxed_slice())
                } else {
                    n.get_function_context(id).into_iter().filter_map(|namespace| match namespace.get_return(){
                        Option::None => None,
                        Option::Some(ref return_expr) => InlinedExpression::new(return_expr, n, func_ctx)
                    }).next()
                } 
            },
        }
    }
}
