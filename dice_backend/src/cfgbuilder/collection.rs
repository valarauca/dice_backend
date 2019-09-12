
use std::collections::{BTreeMap,HashMap};

use super::super::namespace::{BlockExpression,BasicBlock,Namespace};
use super::super::seahasher::DefaultSeaHasher;

use super::expression::{HashedExpression};

/// ExpressionCollection initializes the process of providing some
/// kind of intra-expression linkage. Common identifiers, constants,
/// and expressions are all reduced to hashes. Expressions which contain
/// nested expressions now point to an identifier, not an allocation.
pub struct ExpressionCollection<'a> {
    data: BTreeMap<u64,HashedExpression<'a>>,
    vars: HashMap<&'a str, u64, DefaultSeaHasher>,
    ret: Option<u64>,
}
impl<'a> ExpressionCollection<'a> {

    /// This converts a basic block into a much lower CFG like expression.
    pub fn from_block(block: &BasicBlock<'a>) -> ExpressionCollection<'a> {
        let mut collection = ExpressionCollection {
            data: BTreeMap::default(),
            vars: HashMap::default(),
            ret: None,
        };
        for (name, expr) in block.get_vars() {
            collection.insert_vars(name, expr);
        }
        match block.get_return() {
            &Option::None => unreachable!(),
            &Option::Some(ref expr) => {
                collection.ret = Some(collection.insert_block(expr));
            }
        };
        collection
    }

    pub fn from_namespace(n: &Namespace<'a>) -> Self {
        let mut collection = ExpressionCollection {
            data: BTreeMap::default(),
            vars: HashMap::default(),
            ret: None,
        };
        for (name, expr) in n.get_own_block().into_iter().flat_map(|b| b.get_vars()) {
            collection.insert_vars(name, expr); 
        }
        match n.get_own_block().into_iter().flat_map(|b| b.get_return()).next() {
            Option::None => unreachable!(),
            Option::Some(ref expr) => {
                collection.ret = Some(collection.insert_block(expr));
            }
        };
        collection
    }

    /// each variable is shoved into the map
    fn insert_vars(&mut self, name: &'a str, expr: &BlockExpression<'a>) {
        let identifier = self.insert_block(expr);
        self.vars.insert(name, identifier);
    }

    /// individual expressions are converted to HashedExpression, then inserted
    /// into the internal collection.
    fn insert_block(&mut self, expr: &BlockExpression<'a>) -> u64 {
        let expr = match expr {
            &BlockExpression::FunctionArg(ref name, ref kind) => HashedExpression::FunctionArg(name.clone(), kind.clone()),
            &BlockExpression::ConstantValue(ref value, ref kind) => HashedExpression::ConstantValue(value.clone(), kind.clone()),
            &BlockExpression::ExternalConstant(ref name, ref kind) => HashedExpression::ExternalConstant(name.clone(), kind.clone()),
            &BlockExpression::Var(ref name, ref kind) => HashedExpression::Var(name.clone(), kind.clone()),
            &BlockExpression::Func(ref name, ref args, ref kind) => {
                let arg_refs = args.iter().map(|argument| self.insert_block(argument)).collect::<Vec<u64>>().into_boxed_slice();
                HashedExpression::Func(name.clone(), arg_refs, kind.clone())
            },
            &BlockExpression::Op(ref left, ref op, ref right, ref kind) => {
                let left = self.insert_block(left);
                let right = self.insert_block(right);
                HashedExpression::Op(left, op.clone(), right, kind.clone())
            },
        };
        self.insert_expression(expr)
    }

    fn insert_expression(&mut self, expr: HashedExpression<'a>) -> u64 {
        let key = expr.get_hash();
        self.data.insert(key.clone(), expr);
        key
    }
}
