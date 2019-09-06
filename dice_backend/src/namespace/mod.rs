//! Here we resolve namespacing, and prune some low-hanging
//! fruit errors. The type system is extremely trivial so
//! there is not much to check.
mod block;
pub use self::block::BasicBlock;
mod blockexpression;
pub use self::blockexpression::BlockExpression;
mod namespace;
pub use self::namespace::Namespace;
