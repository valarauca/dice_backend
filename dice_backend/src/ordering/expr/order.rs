
/// Ordering concerns ordering data
#[derive(Default,Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Ordering {
    op_order: u64
}
impl AsRef<Ordering> for Ordering {
    #[inline(always)]
    fn as_ref<'a>(&'a self) -> &'a Ordering {
        self
    }
}
impl AsMut<Ordering> for Ordering {
    #[inline(always)]
    fn as_mut<'a>(&'a mut self) -> &'a mut Ordering {
        self
    }
}
impl OrderingOp for Ordering { }

/// Handles ordering operation values
pub trait OrderingOp: AsRef<Ordering> + AsMut<Ordering> {

    /// set allows for setting an ordering value
    fn set(&mut self, value: u64) {
        self.as_mut().op_order = value;
    }

    /// get allows for fetching the ordering value
    fn get(&self) -> u64 {
        self.as_ref().op_order.clone()
    }

    /// tests if the ordering of two values is the same
    fn equal_ordering<T: OrderingOp>(&self, other: &T) -> bool {
        self.get() == other.get()
    }
}
