use super::super::cfgbuilder::{ExpressionCollection, HashedExpression, Identifier};

/// CallStack manages how namespace expression look ups are handled
/// Namely, that we keep track of _where_ we are within the call stack while doing inlining.
#[derive(Clone)]
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
        assert_eq!(
            self.stack_function_body.len(),
            self.stack_function_name.len()
        );
        let namespace = match self.namespace.get_function_context(id) {
            Option::None => unreachable!(),
            Option::Some(namespace) => namespace,
        };
        self.stack_function_name.push(id.clone());
        self.stack_function_body.push(namespace);
    }

    /// removes a function from the namespace
    pub fn pop(&mut self) {
        assert_eq!(
            self.stack_function_body.len(),
            self.stack_function_name.len()
        );
        self.stack_function_name.pop();
        self.stack_function_body.pop();
    }

    pub fn is_stdlib(&self, id: &Identifier) -> bool {
        self.namespace.is_function_stdlib(id)
    }

    /// provides the returning expression for the current namespace
    pub fn get_return(&self) -> Option<&'b HashedExpression<'a>> {
        assert_eq!(
            self.stack_function_body.len(),
            self.stack_function_name.len()
        );
        self.get_last_index()
            .into_iter()
            .flat_map(|index| self.stack_function_body[index].get_return())
            .chain(self.namespace.get_return())
            .next()
    }

    /// This will look for an expression within the current context
    pub fn get_expr(&self, id: &u64) -> Option<&'b HashedExpression<'a>> {
        self.namespace.get_expr(self.get_context(), id)
    }

    /// Returns the expression that defined a variable
    pub fn get_var(&self, id: &Identifier) -> Option<&'b HashedExpression<'a>> {
        // variable names must be unique
        self.namespace.get_variable(id)
    }

    /// returns the identifier for the current context
    pub fn get_context(&self) -> Option<Identifier> {
        self.get_last_index()
            .into_iter()
            .map(|index| self.stack_function_name[index].clone())
            .next()
    }

    #[inline(always)]
    fn get_last_index(&self) -> Option<usize> {
        match self.stack_function_name.len() {
            0 => None,
            x => Some(x - 1),
        }
    }
}
