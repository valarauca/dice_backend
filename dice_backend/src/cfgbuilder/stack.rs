use super::super::cfgbuilder::{ExpressionCollection, HashedExpression, Identifier};

/// CallStack manages how namespace expression look ups are handled
/// Namely, that we keep track of _where_ we are within the call stack while doing inlining.
#[derive(Clone)]
pub struct CallStack<'a, 'b> {
    namespace: &'b ExpressionCollection<'a>,
    body: Vec<&'b ExpressionCollection<'a>>,
    name: Vec<Identifier>,
    args: Vec<Box<[u64]>>,
    expr: Vec<u64>,
}
impl<'a, 'b> CallStack<'a, 'b> {
    /// This will build a new instance of CallStack from a root
    /// level namespace.
    pub fn new(namespace: &'b ExpressionCollection<'a>) -> CallStack<'a, 'b> {
        CallStack {
            namespace: namespace,
            body: Vec::with_capacity(10),
            name: Vec::with_capacity(10),
            args: Vec::with_capacity(10),
            expr: Vec::with_capacity(10),
        }
    }

    /// push function will modify the internal stack adding another function to the
    /// context
    pub fn push(&mut self, id: &Identifier, expr: &u64) {
        let args = match self.get_expr(expr) {
            Option::Some(HashedExpression::Func(ref looked_up, ref args, _)) => {
                assert_eq!(looked_up, id);
                args.clone()
            }
            _ => _unreachable_panic!(),
        };
        let namespace = match self.namespace.get_function_context(id) {
            Option::None => _unreachable_panic!(),
            Option::Some(namespace) => namespace,
        };
        self.name.push(id.clone());
        self.body.push(namespace);
        self.args.push(args);
        self.expr.push(expr.clone());
    }

    /// removes a function from the namespace
    pub fn pop(&mut self) {
        self.name.pop();
        self.body.pop();
        self.args.pop();
        self.expr.pop();
    }

    pub fn is_stdlib(&self, id: &Identifier) -> bool {
        self.namespace.is_function_stdlib(id)
    }

    pub fn get_function_name(&self, id: &Identifier) -> Option<&'a str> {
        self.namespace.get_function_name(id)
    }

    /// provides the returning expression for the current namespace
    pub fn get_return(&self) -> Option<&'b HashedExpression<'a>> {
        self.get_last_index()
            .into_iter()
            .flat_map(|index| self.body[index].get_return())
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
            .map(|index| self.name[index].clone())
            .next()
    }
    /// returns the expression of the function invocation
    /// we are currently in
    pub fn get_ctx_expr(&self) -> Option<u64> {
        self.get_last_index()
            .into_iter()
            .map(|index| self.expr[index].clone())
            .next()
    }
    /// returns the expression data of the arg's index we're in
    pub fn get_arg_index(&self, arg_index: usize) -> Option<u64> {
        use std::ops::Index;
        self.get_last_index()
            .into_iter()
            .map(|index| self.args.index(index).index(arg_index).clone())
            .next()
    }

    #[inline(always)]
    fn get_last_index(&self) -> Option<usize> {
        match self.name.len() {
            0 => None,
            x => Some(x - 1),
        }
    }
}
