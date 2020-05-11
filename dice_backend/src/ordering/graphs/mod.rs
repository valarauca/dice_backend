mod swap_source;
pub use self::swap_source::SwapSource;

mod add_sink;
pub use self::add_sink::AddSink;

mod remove_sink;
pub use self::remove_sink::RemoveSink;

mod remove;
pub use self::remove::Remover;

mod traits;
pub use self::traits::{Graph, ModifyGraph};

mod inserter;
pub use self::inserter::Inserter;

mod operation;
pub use self::operation::Operation;

mod op_coll;
pub use self::op_coll::Modifications;
