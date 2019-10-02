use super::super::cfgbuilder::{ExpressionCollection, HashedExpression, Identifier};

/// CallStack manages how namespace expression look ups are handled
/// Namely, that we keep track of _where_ we are within the call stack while doing inlining.
pub struct CallStack<'a, 'b> {
    namespace: &'b ExpressionCollection<'a>,
    stack_function_body: Vec<&'b ExpressionCollection<'a>>,
    stack_function_name: Vec<Identifier>,
}
impl<'a, 'b> CallStack<'a, 'b> {
    /// This will build a new instance of CallStack from a root
    /// level namespace.
    pub fn new(namespace: &'b ExpressionCollection<'a>) -> CallStack<'a, 'b> {
        CallStack {
            namespace: namespace,
            stack_function_body: Vec::with_capacity(10),
            stack_function_name: Vec::with_capacity(10),
        }
    }

    /// push function will modify the internal stack adding another function to the
    /// context
    pub fn push(&mut self, id: &Identifier) {
        assert_eq!(self.stack_function_body.len(), self.stack_function_name.len());
        let namespace = match self.namespace.get_function_context(id) {
            Option::None => unreachable!(),
            Option::Some(namespace) => namespace
        };
        self.stack_function_name.push(id.clone());
        self.stack_function_body.push(namespace);
    }

    /// removes a function from the namespace
    pub fn pop(&mut self) {
        assert_eq!(self.stack_function_body.len(), self.stack_function_name.len());
        self.stack_function_name.pop();
        self.stack_function_body.pop();
    }

    /// This will look for an expression within the current context
    pub fn get_expr_current(&self, id: &u64) -> Option<&'b HashedExpression<'a>> {
        self.namespace.get_expr(self.get_context(0), id)
    }

    /// This will look for the an expression within the parent context
    pub fn get_expr_parent(&self, id: &u64) -> Option<&'b HashedExpression<'a>> {
        self.namespace.get_expr(self.get_context(1), id)
    }

    /// Returns the expression that defined a variable
    pub fn get_var(&self, id: &Identifier) -> Option<&'b HashedExpression<'a>> {
        // variable names must be unique
        self.namespace.get_variable(id)
    }

    /// returns the identifier for the current context
    fn get_context(&self, depth: usize) -> Option<Identifier> {
        let length = self.stack_function_name.len();
        if length > depth {
            Some(self.stack_function_name[length-(depth+1)].clone())
        } else {
            None
        }
    }
}
