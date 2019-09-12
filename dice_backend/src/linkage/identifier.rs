

/// Identifier is used to handle the dual namespacing we have going on.
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Identifier {
    TopLevel(u64),
    Interior(u64,u64),
}

