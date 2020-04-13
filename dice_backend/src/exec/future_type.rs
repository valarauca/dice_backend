use std::collections::BTreeMap;
use std::mem::{replace, ManuallyDrop};
use std::pin::Pin;
use std::sync::atomic::{fence, Ordering};
use std::sync::{Arc, Mutex};

use super::super::rayon::iter::once;
use super::{ProbabilityDataType, TupleElement};

/// ProbabilityFuture will eventually return a ProbabilityDataType
#[derive(Clone)]
pub struct ProbabilityFuture {
    data: Arc<Mutex<DataFuture>>,
}
impl ProbabilityFuture {
    /// build a new probability future
    pub fn lambda<F>(lambda: F) -> Self
    where
        F: 'static + Fn() -> ProbabilityDataType,
    {
        Self {
            data: Arc::new(Mutex::new(DataFuture::new(lambda))),
        }
    }

    /// constant is data already known
    pub fn constant(data: TupleElement) -> Self {
        Self {
            data: Arc::new(Mutex::new(DataFuture::Ran(ProbabilityDataType::new(once(
                data,
            ))))),
        }
    }

    /// returns the data of the internal lambda
    pub fn get_data(&self) -> ProbabilityDataType {
        self.data.lock().unwrap().exec()
    }
}

enum DataFuture {
    NotRan(Box<dyn Fn() -> ProbabilityDataType>),
    Ran(ProbabilityDataType),
}
impl DataFuture {
    // bild a new executor
    fn new<F>(lambda: F) -> Self
    where
        F: 'static + Fn() -> ProbabilityDataType,
    {
        DataFuture::NotRan(Box::new(lambda))
    }

    /// check if we need to run the future, or not
    fn exec(&mut self) -> ProbabilityDataType {
        let result = match self {
            &mut DataFuture::Ran(ref data) => {
                // function alrady ran
                return data.clone();
            }
            &mut DataFuture::NotRan(ref lambda) => {
                // now we need to run the future
                (lambda)()
            }
        };
        // update our status as having ran
        replace(self, DataFuture::Ran(result.clone()));
        // return result
        result
    }
}
