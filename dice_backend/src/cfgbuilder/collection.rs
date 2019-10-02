use std::collections::{BTreeMap, HashMap};

use super::super::namespace::{BasicBlock, BlockExpression, Namespace};
use super::super::parser_output::FunctionDeclaration;
use super::super::seahasher::DefaultSeaHasher;

use super::expression::HashedExpression;
use super::identifier::Identifier;

/// ExpressionCollection initializes the process of providing some
/// kind of intra-expression linkage. Common identifiers, constants,
/// and expressions are all reduced to hashes. Expressions which contain
/// nested expressions now point to an identifier, not an allocation.
#[derive(Default)]
pub struct ExpressionCollection<'a> {
    data: BTreeMap<u64, HashedExpression<'a>>,
    var_names: BTreeMap<Identifier, &'a str>,
    vars: BTreeMap<Identifier, u64>,
    functions: BTreeMap<Identifier, ExpressionCollection<'a>>,
    function_signature: BTreeMap<Identifier, FunctionDeclaration<'a>>,
    ret: Option<u64>,
}
impl<'a> ExpressionCollection<'a> {
    /// Takes the existing namespace structure and converts it to an
    /// an expression collection.
    pub fn new(namespace: &Namespace<'a>) -> ExpressionCollection<'a> {
        let mut expression = ExpressionCollection::from_namespace(namespace);
        for (name, block) in namespace.get_all_function_blocks() {
            expression.insert_function(namespace, name, block)
        }
        expression
    }

    /// get the return statement
    pub fn get_return<'b>(&'b self) -> Option<&'b HashedExpression<'a>> {
        self.ret
            .clone()
            .into_iter()
            .flat_map(|ret_id| self.data.get(&ret_id))
            .next()
    }

    /// returns an expression
    pub fn get_expr<'b>(
        &'b self,
        namespace: Option<Identifier>,
        id: &u64,
    ) -> Option<&'b HashedExpression<'a>> {
        namespace
            .into_iter()
            .flat_map(|namespace| self.functions.get(&namespace))
            .flat_map(|func| func.get_expr(None, id))
            .chain(self.data.get(id))
            .next()
    }

    /// returns the body of a function
    pub fn get_function_context<'b>(
        &'b self,
        id: &Identifier,
    ) -> Option<&'b ExpressionCollection<'a>> {
        self.functions.get(id)
    }

    /// returns if the function is or is not part of the stdlib
    pub fn is_function_stdlib(&self, id: &Identifier) -> bool {
        self.function_signature
            .get(id)
            .into_iter()
            .map(|sig| sig.stdlib.clone())
            .next()
            .unwrap_or(false)
    }

    /// get variable returns the hashed expression which defines the variable.
    pub fn get_variable<'b>(&'b self, id: &Identifier) -> Option<&'b HashedExpression<'a>> {
        // try to convert ID into a namespace
        id.defining_namespace()
            .into_iter()
            // look up that namespace's function
            .flat_map(|namespace| self.functions.get(&namespace))
            // look up that variable within the function's namespace
            .flat_map(|func| func.get_variable(id))
            // if we failed to find anything build in a fall back
            // just search the local namespace
            .chain(
                self.vars
                    .get(id)
                    .into_iter()
                    .filter_map(|expr_id| self.get_expr(None, expr_id)),
            )
            // return the first thing we find
            .next()
    }

    /// This converts a basic block into a much lower CFG like expression.
    fn from_block(
        namespace: &'a str,
        block: &BasicBlock<'a>,
        sig: &FunctionDeclaration<'a>,
    ) -> ExpressionCollection<'a> {
        let mut collection = ExpressionCollection::default();
        for (name, expr) in block.get_vars() {
            collection.insert_vars(Some(namespace), name, expr);
        }
        if !sig.stdlib {
            match block.get_return() {
                &Option::None => unreachable!(),
                &Option::Some(ref expr) => {
                    collection.ret = Some(collection.insert_block(Some(namespace), expr));
                }
            };
        }
        collection
    }

    /*
     * Private Functions help in construction
     *
     */

    /// convert the top level namespace to a value.
    fn from_namespace(n: &Namespace<'a>) -> Self {
        let mut collection = ExpressionCollection::default();
        for (name, expr) in n.get_own_block().into_iter().flat_map(|b| b.get_vars()) {
            collection.insert_vars(None, name, expr);
        }
        match n
            .get_own_block()
            .into_iter()
            .flat_map(|b| b.get_return())
            .next()
        {
            Option::None => unreachable!(),
            Option::Some(ref expr) => {
                collection.ret = Some(collection.insert_block(None, expr));
            }
        };
        collection
    }

    fn insert_function(
        &mut self,
        namespace: &Namespace<'a>,
        name: &'a str,
        block: &BasicBlock<'a>,
    ) {
        let id = Identifier::new(None, name);
        let sig = match namespace.get_function(name) {
            Option::None => unreachable!(),
            Option::Some(arg) => arg.clone(),
        };
        let coll = ExpressionCollection::from_block(name, block, &sig);
        self.functions.insert(id, coll);
        self.function_signature.insert(id, sig);
    }

    /// each variable is shoved into the map
    fn insert_vars(
        &mut self,
        namespace: Option<&'a str>,
        name: &'a str,
        expr: &BlockExpression<'a>,
    ) {
        let blocked_id = self.insert_block(namespace, expr);
        let name_id = Identifier::new(namespace, name);
        self.var_names.insert(name_id, name);
        self.vars.insert(name_id, blocked_id);
    }

    /// individual expressions are converted to HashedExpression, then inserted
    /// into the internal collection.
    fn insert_block(&mut self, n: Option<&'a str>, expr: &BlockExpression<'a>) -> u64 {
        let expr = match expr {
            &BlockExpression::FunctionArg(ref name, ref index, ref kind) => {
                let name_id = Identifier::new(n, name);
                HashedExpression::FunctionArg(name_id, index.clone(), kind.clone())
            }
            &BlockExpression::ConstantValue(ref value, ref kind) => {
                HashedExpression::ConstantValue(value.clone(), kind.clone())
            }
            &BlockExpression::ExternalConstant(ref name, ref kind) => {
                let name_id = Identifier::new(None, name);
                HashedExpression::ExternalConstant(name_id, kind.clone())
            }
            &BlockExpression::Var(ref name, ref kind) => {
                let name_id = Identifier::new(n, name);
                HashedExpression::Var(name_id, kind.clone())
            }
            &BlockExpression::Func(ref name, ref args, ref kind) => {
                let name_id = Identifier::new(None, name);
                let arg_refs = args
                    .iter()
                    .map(|argument| self.insert_block(n, argument))
                    .collect::<Vec<u64>>()
                    .into_boxed_slice();
                HashedExpression::Func(name_id, arg_refs, kind.clone())
            }
            &BlockExpression::Op(ref left, ref op, ref right, ref kind) => {
                let left = self.insert_block(n, left);
                let right = self.insert_block(n, right);
                HashedExpression::Op(left, op.clone(), right, kind.clone())
            }
        };
        self.insert_expression(expr)
    }

    fn insert_expression(&mut self, expr: HashedExpression<'a>) -> u64 {
        let key = expr.get_hash();
        self.data.insert(key.clone(), expr);
        key
    }
}
