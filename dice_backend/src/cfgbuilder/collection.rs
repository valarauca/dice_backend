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
    function_names: BTreeMap<Identifier, &'a str>,
    function_signature: BTreeMap<Identifier, FunctionDeclaration<'a>>,
    ret: Option<u64>,
}
impl<'a> ExpressionCollection<'a> {
    pub fn new(namespace: &Namespace<'a>) -> ExpressionCollection<'a> {
        let mut expression = ExpressionCollection::from_namespace(namespace);
        for (name, block) in namespace.get_all_function_blocks() {
            let function_name = Identifier::new(None, name);
            let function_sig = match namespace.get_function(name) {
                Option::None => unreachable!(),
                Option::Some(arg) => arg.clone(),
            };
            expression
                .function_signature
                .insert(function_name.clone(), function_sig);
            expression.functions.insert(
                function_name.clone(),
                ExpressionCollection::from_block(name, block),
            );
            expression
                .function_names
                .insert(function_name.clone(), name);
        }
        expression
    }

    /// This converts a basic block into a much lower CFG like expression.
    fn from_block(namespace: &'a str, block: &BasicBlock<'a>) -> ExpressionCollection<'a> {
        let mut collection = ExpressionCollection::default();
        for (name, expr) in block.get_vars() {
            collection.insert_vars(Some(namespace), name, expr);
        }
        match block.get_return() {
            &Option::None => unreachable!(),
            &Option::Some(ref expr) => {
                collection.ret = Some(collection.insert_block(Some(namespace), expr));
            }
        };
        collection
    }

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
