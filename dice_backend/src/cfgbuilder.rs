
use super::namespace::*;
use super::seahasher::DefaultSeaHasher;

pub fn build_cfg<'a>(namespace: &Namespace<'a>) -> Result<(),String> {
    let analysis = match namespace.get_own_block() {
        Option::None => return Err(format!("program has no analysis stage. Nothing to output")),
        Option::Some(bb) => match bb.get_return() {
            Option::None => return Err(format!("program has no analysis stage. Nothing to output")),
            Option::Some(expr) => expr,
        }
    };
}
