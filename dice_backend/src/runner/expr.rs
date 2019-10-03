use super::super::cfgbuilder::{CallStack, ExpressionCollection, HashedExpression, Identifier};
use super::super::parser_output::{Literal, Operation};

/// Inlined Expression contains the very base values
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InlinedExpression<'a> {
    StdLibFunc(&'a str, Box<InlinedExpression<'a>>),
    Operation(
        Box<InlinedExpression<'a>>,
        Operation,
        Box<InlinedExpression<'a>>,
    ),
    Constant(Literal<'a>),
}
impl<'a> InlinedExpression<'a> {
    /// Build an inlined expression from context
    pub fn new<'b>(
        expr: &'b HashedExpression<'a>,
        stack: &mut CallStack<'a, 'b>,
    ) -> Option<InlinedExpression<'a>> {
        let hash = expr.get_hash();
        match expr {
            &HashedExpression::ConstantValue(ref literal, _) => {
                Some(InlinedExpression::Constant(literal.clone()))
            }
            &HashedExpression::ExternalConstant(ref id, _) | &HashedExpression::Var(ref id, _) => {
                // resolve the expression that defines the variable
                // convert that recursively
                stack
                    .get_var(id)
                    .into_iter()
                    .flat_map(|expr| InlinedExpression::new(expr, stack))
                    .next()
            }
            &HashedExpression::FunctionArg(ref name, ref index, _) => {
                let context = stack.get_context().unwrap();
                stack.pop();
                // todo
                //let function_
                let output = stack
                    .get_expr(&hash)
                    .into_iter()
                    .flat_map(HashedExpression::get_func_arg(index))
                    .flat_map(|hash| stack.get_expr(&hash))
                    .flat_map(|expr| InlinedExpression::new(expr, stack))
                    .next();
                stack.push(&context);
                output
            }
            &HashedExpression::Func(ref id, ref args, _) => {
                stack.push(id);
                let output = stack
                    .get_return()
                    .into_iter()
                    .flat_map(|expr| InlinedExpression::new(expr, stack))
                    .next();
                stack.pop();
                output
            }
        }
    }
}
