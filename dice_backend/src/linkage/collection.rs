
use std::collections::{BTreeMap};

use super::super::namespace::{
    Namespace, BasicBlock,
};

use super::identifier::Identifier;
use super::expr::LinkedExpression;


pub fn linkaged(n: &Namespace<'a>) {
    let block = n.get_own_block().unwrap();
    
}
